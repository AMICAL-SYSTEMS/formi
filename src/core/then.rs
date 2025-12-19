//! 6.1.2270 THEN
//! CORE
//! 
//!     Interpretation: Interpretation semantics for this word are undefined.
//! 
//!     Compilation: ( C: orig -- )
//! 
//! Append the run-time semantics given below to the current definition. 
//! Resolve the forward reference orig using the location of the appended 
//! run-time semantics.
//! 
//!     Run-time: ( -- )
//! 
//! Continue execution. 
//! 
//! AMICAL NOTE: See [`crate::core::if::If`]
//! 
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2270`]
crate::define_word!(Then, "THEN", |_it, _tks| {
    Ok(())
});
