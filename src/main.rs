#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod stack;
pub mod tokens;

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
