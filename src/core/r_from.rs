//! 6.1.2060 R>
//! r-from CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Execution: ( -- x ) ( R:  x -- )
//!
//! Move x from the return stack to the data stack.
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.2060]
crate::define_word!(RFrom, "R>", |it, _tks| {
    let x = it.pop_last_return_stack()?;

    it.stack.push(x);

    Ok(())
});
