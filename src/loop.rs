use core::ops::Range;

use crate::{error::RuntimeError, stack::CellStack};

/// Loop-control parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopSys {
    pub range: Range<u64>,
    pub current_index: u64,
}

impl LoopSys {
    pub fn to_stack(&self, stack: &mut CellStack) {
        stack.push(self.range.start);
        stack.push(self.range.end);
        stack.push(self.current_index);
    }

    pub fn pop_from_stack(stack: &mut CellStack) -> Result<Self, RuntimeError> {
        let idx = stack.pop().ok_or(RuntimeError::EmptyStack)?;
        let end = stack.pop().ok_or(RuntimeError::EmptyStack)?;
        let start = stack.pop().ok_or(RuntimeError::EmptyStack)?;

        Ok(Self {
            range: start..end,
            current_index: idx,
        })
    }
}
