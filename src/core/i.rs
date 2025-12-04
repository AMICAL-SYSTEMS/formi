use crate::{error::RuntimeError, interpreter::InterpreterState};

crate::define_word!(I, "I", |it, _tks| {
    if let InterpreterState::Loop(_, i) = it.state {
        it.stack.push(i);
    } else {
        return Err(RuntimeError::NotInLoop);
    }

    Ok(())
});
