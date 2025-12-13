//! 6.1.1730 J
//! CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Execution: ( -- n|u ) ( R: loop-sys1 loop-sys2 -- loop-sys1 loop-sys2 )
//!
//! n|u is a copy of the next-outer loop index. An ambiguous condition exists if the loop control parameters of the next-outer loop, loop-sys1, are unavailable.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1730`]
use crate::{error::RuntimeError, r#loop::LoopSys};

// TODO: Get `it.return_stack[-4]` directly?
crate::define_word!(J, "J", |it, _tks| {
    let loop_sys2 = LoopSys::pop_from_stack(&mut it.return_stack)?;

    // ASSUMPTION: If loop-sys1 ([`crate::loop::LoopSys`]) is at the
    // top of the stack, then the current loop index is the topmost
    // elememnt of the stack.
    //
    // This lets us avoid unnecessarily pop-ing loop-sys out.
    let idx = it.return_stack.peek().ok_or(RuntimeError::EmptyStack)?;

    // Put loop-sys2 back to stack
    loop_sys2.to_stack(&mut it.return_stack);

    it.stack.push(idx);

    Ok(())
});
