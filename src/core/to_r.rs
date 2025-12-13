//! 6.1.0580 >R
//! to-r CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Execution: ( x -- ) ( R:  -- x )
//!
//! Move x to the return stack.
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.2070]
crate::define_word!(ToR, ">R", |it, _tks| {
    let x = it.pop_last_stack()?;

    it.return_stack.push(x);

    Ok(())
});
