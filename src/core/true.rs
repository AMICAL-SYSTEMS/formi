use crate::types::TRUE_FLAG;

crate::define_word!(True, "true", |it, _tks| {
    it.stack.push(TRUE_FLAG);
    Ok(())
});
