//! 6.1.1805 LSHIFT
//! l-shift CORE
//!
//!     ( x1 u -- x2 )
//!
//! Perform a logical left shift of u bit-places on x1, giving x2. Put zeroes
//! into the least significant bits vacated by the shift. An ambiguous condition
//! exists if u is greater than or equal to the number of bits in a cell.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1805`]

use crate::types::{Cell, UnsignedInteger};

crate::define_word!(LShift, "LSHIFT", |it, _tks| {
    let u: UnsignedInteger = it.pop_last_stack()? as _;
    let x1: Cell = it.pop_last_stack()?;
    let x2: Cell = x1.wrapping_shl(u as u32);
    it.stack.push(x2);

    Ok(())
});
