//! 6.1.2070 R@
//! r-fetch CORE
//! 
//!     Interpretation: Interpretation semantics for this word are undefined.
//! 
//!     Execution: ( -- x ) ( R:  x -- x )
//! 
//! Copy x from the return stack to the data stack. 
//!
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.2070]

use crate::error::RuntimeError;
crate::define_word!(RFetch, "R@", |it, _tks| {
    let x = it.return_stack.peek().ok_or(RuntimeError::EmptyStack)?;

    it.stack.push(x);

    Ok(())
});
