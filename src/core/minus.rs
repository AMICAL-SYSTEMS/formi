crate::define_word!(Minus, "-", |it, _tks| {
    let a = it.pop_last_stack()?;
    let b = it.pop_last_stack()?;

    it.stack.push(a - b);

    Ok(())
});
