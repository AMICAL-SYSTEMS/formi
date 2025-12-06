//! 6.1.0290 1+
//! one-plus CORE
//!
//!     ( n1|u1 -- n2|u2 )
//!
//! Add one (1) to n1|u1 giving the sum n2|u2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0290`]
crate::define_word!(OnePlus, "1+", |it, _tks| {
    let n1u1 = it.pop_last_stack()?;

    it.stack.push(n1u1 + 1);

    Ok(())
});
