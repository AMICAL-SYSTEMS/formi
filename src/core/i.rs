//! 6.1.1680 I
//! CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Execution: ( -- n|u ) ( R:  loop-sys -- loop-sys )
//!
//! n|u is a copy of the current (innermost) loop index. An ambiguous condition exists if the loop control parameters are unavailable.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1680`]
use crate::error::RuntimeError;

crate::define_word!(I, "I", |it, _tks| {
    // ASSUMPTION: If loop-sys ([`crate::loop::LoopSys`]) is at the 
    // top of the stack, then the current loop index is the topmost 
    // elememnt of the stack.
    //
    // This lets us avoid unnecessarily pop-ing loop-sys out.
    let idx = it.return_stack.peek().ok_or(RuntimeError::EmptyStack)?;
    it.stack.push(idx);

    Ok(())
});
