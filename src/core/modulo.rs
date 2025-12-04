use crate::error::RuntimeError;

crate::define_word!(Modulo, "MOD", |it, _tks| {
    let a = it.pop_last_stack()?;
    let b = it.pop_last_stack()?;

    if b == 0 {
        return Err(RuntimeError::DivisionByZero);
    } else {
        it.stack.push(a.rem_euclid(b));
    }

    Ok(())
});
