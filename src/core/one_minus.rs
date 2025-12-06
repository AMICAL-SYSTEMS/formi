//! 6.1.0300 1-
//! one-minus CORE
//! 
//!     ( n1|u1 -- n2|u2 )
//! 
//! Subtract one (1) from n1|u1 giving the difference n2|u2. 
//! 
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0300`]
crate::define_word!(OneMinus, "1-", |it, _tks| {
    let n1u1 = it.pop_last_stack()?;

    it.stack.push(n1u1 - 1);

    Ok(())
});
