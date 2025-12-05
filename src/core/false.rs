use crate::types::FALSE_FLAG;

crate::define_word!(False, "false", |it, _tks| {
    it.stack.push(FALSE_FLAG);
    Ok(())
});
