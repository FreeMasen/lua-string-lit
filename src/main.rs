use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use analisar::aware::ast::{
    Block, ExpListItem, Expression, Field, Statement, SuffixedProperty, Table,
};
use bstr::BString;
use clap::Parser;
use serde::{Serialize, Serializer};

#[derive(Debug, Parser)]
struct Args {
    project_root: PathBuf,
}
#[derive(Debug, Clone, Serialize)]
#[allow(unused)]
struct StringLiteral {
    #[serde(serialize_with = "StringLiteral::ser_value")]
    value: BString,
    start_offset: usize,
    end_offset: usize,
}

impl StringLiteral {
    fn ser_value<S>(value: &BString, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&format!("{}", value))
    }
}

type ProjectMap = HashMap<PathBuf, Vec<StringLiteral>>;

fn main() {
    let args = Args::parse();
    if !args.project_root.exists() {
        eprintln!("Inaccessible project root: {}", args.project_root.display());
        std::process::exit(1);
    }
    let mut all = ProjectMap::new();
    for entry in walkdir::WalkDir::new(args.project_root)
        .contents_first(true)
        .into_iter()
        .filter_map(|p| {
            let p = p.ok()?;
            if p.file_type().is_file() && p.path().extension()? == "lua" {
                return Some(p);
            }
            None
        })
    {
        let strs = collect_file(entry.path());
        all.insert(entry.path().to_path_buf(), strs);
    }
    println!("{}", serde_json::to_string_pretty(&all).unwrap());
}

fn collect_file(path: &Path) -> Vec<StringLiteral> {
    let contents = std::fs::read(path).unwrap();
    let mut parser = analisar::aware::Parser::new(&contents);
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
        Statement::Assignment { values, .. } => collect_expr_list(values),
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
        _ => Vec::new(),
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
        Expression::LiteralString(lit) => vec![StringLiteral {
            start_offset: lit.span.start,
            end_offset: lit.span.end,
            value: BString::new(lit.value.to_vec()),
        }],
        Expression::FunctionDef(body) => {
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
                analisar::aware::ast::Args::Table(t) => ret.extend(collect_table(t)),
                analisar::aware::ast::Args::String(lit) => ret.push(StringLiteral {
                    start_offset: lit.span.start,
                    end_offset: lit.span.end,
                    value: BString::new(lit.value.to_vec()),
                }),
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
        _ => Vec::new(),
    }
}

fn collect_table(t: &Table) -> Vec<StringLiteral> {
    let mut ret = Vec::new();
    for field in &t.field_list {
        match field {
            Field::Record { name, value, .. } => {
                ret.extend(collect_expression(name));
                ret.extend(collect_expression(value));
            }
            Field::List { value, .. } => ret.extend(collect_expression(value)),
        }
    }
    ret
}
