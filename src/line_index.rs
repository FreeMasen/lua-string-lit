/// Pre-cached line/column lookup table for a string slice.
pub struct LineIndex {
    lines: Vec<usize>,
}

impl LineIndex {
    pub fn new<T>(src: T) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut lines = vec![0];
        let mut iter = src.as_ref().into_iter().enumerate().peekable();
        while let Some((i, &byte)) = iter.next() {
            if byte == b'\r' {
                if let Some((_, next)) = iter.peek()
                    && **next == b'\n'
                {
                    let (i, _) = iter.next().unwrap();
                    lines.push(i + 1)
                } else {
                    lines.push(i + 1)
                }
            }
            if byte == b'\n' {
                lines.push(i + 1)
            }
        }
        Self { lines }
    }

    /// Looks up the 1-based line and column numbers of the specified byte index.
    ///
    /// Returns a tuple with the line number first, then column number.
    ///
    /// # Example
    /// ```rust
    /// use line_col::*;
    /// let text = "One\nTwo";
    /// let lookup = LineColLookup::new(text);
    /// assert_eq!(lookup.get(0), (1, 1)); // 'O' (line 1, col 1)
    /// assert_eq!(lookup.get(1), (1, 2)); // 'n' (line 1, col 2)
    /// assert_eq!(lookup.get(2), (1, 3)); // 'e' (line 1, col 3)
    /// assert_eq!(lookup.get(4), (2, 1)); // 'T' (line 2, col 1)
    /// assert_eq!(lookup.get(5), (2, 2)); // 'w' (line 2, col 2)
    /// assert_eq!(lookup.get(6), (2, 3)); // 'o' (line 2, col 3)
    /// assert_eq!(lookup.get(7), (2, 4)); // <end> (line 2, col 4)
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the length of the input `&str`.
    ///
    /// # Notes
    /// This function uses a binary search to locate the line on which `index` resides.
    /// This means that it runs in approximately O(log n) time.
    pub fn get(&self, index: usize) -> (usize, usize) {
        // if index > self.src.len() {
        //     panic!("Index cannot be greater than the length of the input slice.");
        // }

        let mut line_range = 0..self.lines.len();
        while line_range.end - line_range.start > 1 {
            let range_middle = line_range.start + (line_range.end - line_range.start) / 2;
            let (left, right) = (line_range.start..range_middle, range_middle..line_range.end);
            // Check which line window contains our character index
            if (self.lines[left.start]..self.lines[left.end]).contains(&index) {
                line_range = left;
            } else {
                line_range = right;
            }
        }

        let line_start_index = self.lines[line_range.start];
        let line = line_range.start + 1;
        let col = index - line_start_index + 1;

        (line, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_str() {
        let text = "";
        let lookup = LineIndex::new(text);
        assert_eq!(lookup.get(0), (1, 1));
    }

    #[test]
    fn line_col_iter_by_codepoints() {
        let text = "a\nab\nabc";
        let lookup = LineIndex::new(text);
        let lcs: Vec<(usize, usize)> = (0..text.len()).map(|v| lookup.get(v)).collect();
        insta::assert_debug_snapshot!(lcs);
    }

    #[test]
    fn line_col_iter_by_codepoints_crlf() {
        let text = "a\r\nab\r\nabc";
        let lookup = LineIndex::new(text);
        let lcs: Vec<(usize, usize)> = (0..text.len()).map(|v| lookup.get(v)).collect();
        insta::assert_debug_snapshot!(lcs);
    }

    #[test]
    fn emoji_text_by_codepoints() {
        let text = "The 👨‍👩‍👦 emoji is made of 5 code points and 18 bytes in UTF-8.";
        let lookup = LineIndex::new(text);
        assert_eq!(lookup.get(4), (1, 5));
        assert_eq!(lookup.get(22), (1, 23));
    }
}
