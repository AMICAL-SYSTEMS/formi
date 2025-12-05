use crate::types::Cell;

#[derive(Debug, Default)]
pub struct DataStack(alloc::vec::Vec<Cell>);

impl DataStack {
    #[inline]
    pub fn push(&mut self, value: Cell) {
        self.0.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<Cell> {
        self.0.pop()
    }

    #[inline]
    pub fn peek(&self) -> Option<Cell> {
        self.0.last().copied()
    }
}
