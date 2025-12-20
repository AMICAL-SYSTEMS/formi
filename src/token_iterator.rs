use std::vec::Vec;

#[derive(Debug)]
pub struct TokenIterator<'a> {
    cursor: usize,
    tokens: Vec<&'a str>,
    /// This variable stores how many layers deep inside
    /// an inline comment we're in. This lets us know
    /// when the last closing parenthesis ) is (which is
    /// when this variable is equal to 0).
    nested_inline_comment_count: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn new(tokens: &'a str) -> TokenIterator<'a> {
        let tokens: Vec<&'a str> = tokens
            .as_bytes()
            .split(|s| s.is_ascii_whitespace())
            .filter(|x| !x.is_empty())
            .map(|b| {
                // SAFETY: bytes are constructed from a String
                unsafe { str::from_utf8_unchecked(b) }
            })
            // Comments in Forth begin with a backslash. Ignore all tokens after a backslash.
            .take_while(|token| token != &"\\")
            .collect();

        Self {
            cursor: 0,
            tokens,
            nested_inline_comment_count: 0,
        }
    }

    #[inline]
    pub fn shift_cursor(&mut self, n: isize) {
        self.cursor += n.cast_unsigned();
    }

    #[inline]
    pub fn move_cursor(&mut self, n: isize) {
        self.cursor = n.cast_unsigned();
    }

    #[inline]
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    #[inline]
    pub fn reset_cursor(&mut self) {
        self.move_cursor(0);
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;

        loop {
            let tok = self.tokens.get(self.cursor - 1).copied();

            match tok {
                Some("(") => {
                    self.nested_inline_comment_count += 1;
                }
                Some(")") => {
                    self.nested_inline_comment_count =
                        self.nested_inline_comment_count.saturating_sub(1);
                }
                other => {
                    if self.nested_inline_comment_count == 0 {
                        return other;
                    }
                }
            }

            self.cursor += 1;
        }
    }
}
