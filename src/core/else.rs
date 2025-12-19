//! 6.1.1310 ELSE
//! CORE
//!
//!     Interpretation: Interpretation semantics for this word are undefined.
//!
//!     Compilation: ( C: orig1 -- orig2 )
//!
//! Put the location of a new unresolved forward reference orig2 onto the control
//! flow stack. Append the run-time semantics given below to the current definition.
//! The semantics will be incomplete until orig2 is resolved (e.g., by THEN).
//! Resolve the forward reference orig1 using the location following the appended run-time semantics.
//!
//!     Run-time: ( -- )
//!
//! Continue execution at the location given by the resolution of orig2.
//!
//! AMICAL NOTE: See [`crate::core::if::If`]
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1310`]

crate::define_word!(Else, "ELSE", |_it, _tks| { Ok(()) });
