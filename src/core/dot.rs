use alloc::format;

use crate::error::RuntimeError;

crate::define_word!(Dot, ".", |it, _tks| {
    let a = it.stack.pop().ok_or(RuntimeError::EmptyStack)?;
    it.stdout_write(format!("{a}\n").as_bytes())?;

    Ok(())
});
