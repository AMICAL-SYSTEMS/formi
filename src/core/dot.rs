use std::string::ToString;

use crate::error::RuntimeError;

crate::define_word!(Dot, ".", |it, _tks| {
    let a = it.stack.pop().ok_or(RuntimeError::EmptyStack)?;
    it.stdout_write(a.to_string().as_bytes())?;

    Ok(())
});
