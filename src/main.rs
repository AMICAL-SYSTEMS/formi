#![no_std]
//! `formi` is a Forth interpreter.
//!
//! We aim to provide at least support for the `core` set of words
//! defined by the Forth 94 standard available at [`https://www.taygeta.com/forth/dpans.html`].
//!
//! However, it is not our goal to provide 100% compliance with this standard, and
//! we may choose to implement the language differently should we feel that it is more appropriate to.
//!
//! A subset of Core words, as well as their definitions, are available at [`crate::core`].

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod core;
pub mod error;
pub mod stack;
pub mod types;

#[cfg(feature = "repl")]
pub mod interpreter;
#[cfg(feature = "repl")]
pub mod repl;

fn main() {
    if cfg!(feature = "repl") {
        repl::repl();
    }
}
