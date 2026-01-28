use std::{
    collections::HashMap, path::PathBuf, str::FromStr
};

use analisar::aware::ast::{
    Block, ExpListItem, Expression, Field, LiteralString, Statement, SuffixedProperty,
};
use bstr::BString;
use clap::Parser;
use serde::Serialize;

use crate::line_index::LineIndex;

mod line_index;

#[derive(Debug, Parser)]
struct Args {
    project_root: PathBuf,
    #[arg(long = "format", short = 'f', default_value_t = OutputFormat::Json)]
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Json,
    Text,
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "json" => OutputFormat::Json,
            "text" => OutputFormat::Text,
            fmt => return Err(format!("unknown format `{fmt}`")),
        })
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Json => "json",
            Self::Text => "text",
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[allow(unused)]
struct StringLiteral {
    value: BString,
    start_offset: usize,
    end_offset: usize,
}

impl<'a> From<&LiteralString<'a>> for StringLiteral {
    fn from(lit: &LiteralString<'a>) -> Self {
        let start_offset = lit.span.start + 1;
        let end_offset = lit.span.end - 1;
        StringLiteral {
            start_offset,
            end_offset,
            value: BString::new(lit.value[1..lit.value.len() - 1].to_vec()),
        }
    }
}

fn main() {
    let args = Args::parse();
    if !args.project_root.exists() {
        eprintln!("Inaccessible project root: {}", args.project_root.display());
        std::process::exit(1);
    }
    let mut all: HashMap<String, Vec<String>> = HashMap::new();
    for entry in walkdir::WalkDir::new(args.project_root).contents_first(true) {
        let Ok(entry) = entry else {
            continue;
        };
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().is_none_or(|ext| ext != "lua") {
            continue;
        }
        let path = entry.path();
        let contents = std::fs::read(path).unwrap();
        let strs = collect_bytes(&contents);
        let index = LineIndex::new(&contents);
        for s in strs {
            let (line, col) = index.get(s.start_offset);
            all.entry(s.value.to_string())
                .and_modify(|v| v.push(format!("{}:{line}:{col}", entry.path().display())))
                .or_insert_with(|| {
                    vec![format!("{}:{line}:{col}", entry.path().display())]
                });
        }
    }
    match args.format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&all).unwrap());
        }
        OutputFormat::Text => {
            for (s, paths) in all.iter() {
                println!("{s}:");
                for path in paths {
                    println!("\t{path}");
                }
            }
        }
    }
}

fn collect_bytes<'a>(contents: &'a [u8]) -> Vec<StringLiteral> {
    let mut parser = analisar::aware::Parser::new(contents);
    let mut ret = Vec::new();
    while let Some(stmt) = parser.next() {
        let stmt = stmt.unwrap();
        ret.extend(collect_statement(&stmt.statement));
    }
    ret
}

fn collect_statement(stmt: &Statement) -> Vec<StringLiteral> {
    match stmt {
        Statement::Expression(expr) => collect_expression(expr),
        Statement::Assignment {
            targets, values, ..
        } => {
            let mut ret = collect_expr_list(targets);
            ret.extend(collect_expr_list(values));
            ret
        }
        Statement::Do { block, .. } => collect_block(block),
        Statement::While { exp, block, .. } => {
            let mut ret = collect_expression(exp);
            ret.extend(collect_block(block));
            ret
        }
        Statement::Repeat { block, exp, .. } => {
            let mut ret = collect_block(block);
            ret.extend(collect_expression(exp));
            ret
        }
        Statement::If(if_stmt) => {
            let mut ret = collect_expression(&if_stmt.test);
            ret.extend(collect_block(&if_stmt.block));
            for else_if in &if_stmt.else_ifs {
                ret.extend(collect_expression(&else_if.test));
                ret.extend(collect_block(&else_if.block));
            }
            if let Some(block) = &if_stmt.catch_all {
                ret.extend(collect_block(block));
            }
            ret
        }
        Statement::For(for_stmt) => {
            let mut ret = collect_expression(&for_stmt.init);
            ret.extend(collect_expression(&for_stmt.limit));
            if let Some(step) = &for_stmt.step {
                ret.extend(collect_expression(step));
            }
            ret.extend(collect_block(&for_stmt.block));
            ret
        }
        Statement::ForIn(for_stmt) => collect_block(&for_stmt.block),
        Statement::Function { body, .. } => collect_block(&body.block),
        Statement::Return(ret_stmt) => collect_expr_list(&ret_stmt.exprs),
        Statement::Empty(_)
        | Statement::Label { .. }
        | Statement::Break(_)
        | Statement::GoTo { .. } => Vec::new(),
    }
}

fn collect_block(block: &Block) -> Vec<StringLiteral> {
    let mut ret = Vec::new();
    for stmt in &block.0 {
        ret.extend(collect_statement(stmt));
    }
    ret
}

fn collect_expr_list(expr_list: &[ExpListItem]) -> Vec<StringLiteral> {
    let mut ret = Vec::new();
    for expr in expr_list {
        if let ExpListItem::Expr(expr) = expr {
            ret.extend(collect_expression(expr));
        }
    }
    ret
}

fn collect_expression(expr: &Expression) -> Vec<StringLiteral> {
    match expr {
        Expression::LiteralString(lit) => vec![lit.into()],
        Expression::FunctionDef { keyword: _, body } => {
            let mut ret = Vec::new();
            for stmt in &body.block.0 {
                ret.extend(collect_statement(stmt));
            }
            ret
        }
        Expression::Parened { expr, .. } => collect_expression(expr),
        Expression::BinOp { left, right, .. } => {
            let mut ret = collect_expression(left);
            ret.extend(collect_expression(right));
            ret
        }
        Expression::UnaryOp { exp, .. } => collect_expression(exp),
        Expression::FuncCall(call) => {
            let mut ret = collect_expression(&call.prefix);
            match &call.args {
                analisar::aware::ast::Args::ExpList { exprs, .. } => {
                    ret.extend(collect_expr_list(exprs));
                }
                analisar::aware::ast::Args::Table(t) => ret.extend(collect_table(&t.field_list)),
                analisar::aware::ast::Args::String(lit) => ret.push(lit.into()),
            }
            ret
        }
        Expression::Suffixed(suf) => {
            let mut ret = collect_expression(&suf.subject);
            if let SuffixedProperty::Computed { expr, .. } = &suf.property {
                ret.extend(collect_expression(expr));
            }
            ret
        }
        Expression::TableCtor(table) => collect_table(&table.field_list),
        Expression::Nil(_)
        | Expression::False(_)
        | Expression::True(_)
        | Expression::Numeral(_)
        | Expression::Name(_)
        | Expression::VarArgs(_) => Vec::new(),
    }
}

fn collect_table(field_list: &[Field]) -> Vec<StringLiteral> {
    let mut ret = Vec::new();
    for field in field_list {
        match field {
            Field::Record { name, value, .. } => {
                ret.extend(collect_expression(&name.expr));
                ret.extend(collect_expression(value));
            }
            Field::List { value, .. } => ret.extend(collect_expression(value)),
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let mut strs = collect_bytes(b"print'hello world'");
        let hw = strs.pop().unwrap();
        assert!(strs.is_empty(), "{strs:?} was not empty");
        assert_eq!(hw.start_offset, 6);
        assert_eq!(&hw.value.to_string(), "hello world");
        assert_eq!(hw.end_offset, 17);
    }

    #[test]
    fn random_smattering() {
        const ASSIGN: &str = "value";
        const ARGUMENT: &str = "arg";
        const TABLE_KEY: &str = "key";
        const TABLE_VALUE: &str = "field";
        const TABLE_ARG_KEY: &str = "arg_key";
        const TABLE_ARG_VALUE: &str = "arg_field";
        const FIELD_ASSIGN_KEY: &str = "assign-key";
        const FIELD_ASSIGN_VALUE: &str = "assign-value";
        env_logger::builder().is_test(true).try_init().ok();
        let lua = format!(
            "\
local v = {ASSIGN}\n\
print({ARGUMENT})\n\
local t = {{ [{TABLE_KEY}] = {TABLE_VALUE} }} \n\
print{{ [{TABLE_ARG_KEY}] = {TABLE_ARG_VALUE} }} \n\
t[{FIELD_ASSIGN_KEY}] = {FIELD_ASSIGN_VALUE} \n\
        "
        );
        println!("lua: {lua}");
        let mut strs = collect_bytes(lua.as_bytes());
        println!("{strs:?}");
        let expected = &[
            ASSIGN,
            ARGUMENT,
            TABLE_KEY,
            TABLE_VALUE,
            TABLE_ARG_KEY,
            TABLE_ARG_VALUE,
            FIELD_ASSIGN_KEY,
            FIELD_ASSIGN_VALUE,
        ];
        for (parsed, expected) in strs.drain(..).zip(expected.iter()) {
            assert_eq!(parsed.value.to_string(), *expected);
        }
        assert!(strs.is_empty(), "{strs:?} was not empty")
    }
}
