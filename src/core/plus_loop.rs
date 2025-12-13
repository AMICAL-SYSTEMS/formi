//! 6.1.0140 +LOOP
//! plus-loop CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Compilation: ( C: do-sys -- )
//!
//! Append the run-time semantics given below to the current definition. Resolve the destination of all unresolved occurrences of LEAVE between the location given by do-sys and the next location for a transfer of control, to execute the words following +LOOP.
//!
//!     Run-time: ( n -- ) ( R: loop-sys1 -- | loop-sys2 )
//!
//! An ambiguous condition exists if the loop control parameters are unavailable. Add n to the loop index. If the loop index did not cross the boundary between the loop limit minus one and the loop limit, continue execution at the beginning of the loop. Otherwise, discard the current loop control parameters and continue execution immediately following the loop.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0140`]
crate::define_word!(PlusLoop, "+LOOP", |_it, _tks| { Ok(()) });
