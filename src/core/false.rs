use crate::types::FALSE_FLAG;

crate::define_word!(False, "FALSE", |it, _tks| {
    it.stack.push(FALSE_FLAG);
    Ok(())
});
