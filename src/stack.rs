use crate::types::Cell;

#[derive(Debug, Default)]
pub struct CellStack(alloc::vec::Vec<Cell>);

pub type DataStack = CellStack;
pub type ReturnStack = CellStack;

impl CellStack {
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
