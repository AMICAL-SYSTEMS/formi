//! 6.1.1260 DROP
//! CORE
//!
//!     ( x -- )
//!
//! Remove x from the stack.
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.1260]

crate::define_word!(Drop, "DROP", |it, _tks| {
    it.pop_last_stack()?;

    Ok(())
});
