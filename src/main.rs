#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod core;
pub mod error;
pub mod stack;

#[cfg(feature = "repl")]
pub mod interpreter;
#[cfg(feature = "repl")]
pub mod repl;

pub const DELIM: &str = " ";

fn main() {
    if cfg!(feature = "repl") {
        repl::repl();
    }
}
