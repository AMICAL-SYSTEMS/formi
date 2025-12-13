//! 6.1.1760 LEAVE
//! CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Execution: ( -- ) ( R: loop-sys -- )
//!
//! Discard the current loop control parameters. An ambiguous condition exists if they are unavailable. Continue execution immediately following the innermost syntactically enclosing DO ... LOOP or DO ... +LOOP.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1760`]
use crate::r#loop::LoopSys;

crate::define_word!(Leave, "LEAVE", |it, _tks| {
    // Exit loop by popping loop control parameters.
    let _ = LoopSys::pop_from_stack(&mut it.return_stack)?;

    Ok(())
});
