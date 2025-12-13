//! 6.1.1800 LOOP
//! CORE
//! 
//!     Interpretation: Interpretation semantics for this word are undefined.
//! 
//!     Compilation: ( C: do-sys -- )
//! 
//! Append the run-time semantics given below to the current definition. Resolve the destination of all unresolved occurrences of LEAVE between the location given by do-sys and the next location for a transfer of control, to execute the words following the LOOP.
//! 
//!     Run-time: ( -- ) ( R:  loop-sys1 --  | loop-sys2 )
//! 
//! An ambiguous condition exists if the loop control parameters are unavailable. Add one to the loop index. If the loop index is then equal to the loop limit, discard the loop parameters and continue execution immediately following the loop. Otherwise continue execution at the beginning of the loop. 
//! 
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1800`]
crate::define_word!(Loop, "LOOP", |_it, _tks| { Ok(()) });
