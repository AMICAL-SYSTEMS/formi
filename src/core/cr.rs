crate::define_word!(Cr, "CR", |it, _tks| {
    it.stdout_write(b"\n")?;

    Ok(())
});
