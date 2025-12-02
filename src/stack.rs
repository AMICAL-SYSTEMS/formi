#[derive(Debug, Default)]
pub struct Stack(alloc::vec::Vec<u64>);

impl Stack {
    pub fn push(&mut self, value: u64) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<u64> {
        self.0.last().copied()
    }
}
