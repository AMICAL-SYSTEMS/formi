use std::vec::Vec;

pub struct TokenIterator<'a> {
    cursor: usize,
    tokens: Vec<&'a str>,
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
            .collect();

        Self { cursor: 0, tokens }
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
        self.tokens.get(self.cursor - 1).copied()
    }
}

impl<'a> DoubleEndedIterator for TokenIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.cursor -= 1;
        self.tokens.get(self.cursor + 1).copied()
    }
}
