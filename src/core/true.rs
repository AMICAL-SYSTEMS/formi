use crate::types::TRUE_FLAG;

crate::define_word!(True, "TRUE", |it, _tks| {
    it.stack.push(TRUE_FLAG);
    Ok(())
});
