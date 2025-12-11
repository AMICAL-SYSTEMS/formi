//! 6.1.2370 UM/MOD
//! u-m-slash-mod CORE
//!
//!     ( ud u1 -- u2 u3 )
//!
//! Divide ud by u1, giving the quotient u3 and the remainder u2. All values
//! and arithmetic are unsigned. An ambiguous condition exists if u1 is zero or
//! if the quotient lies outside the range of a single-cell unsigned integer.
//!
//! AMICAL NOTE: Returns a runtime error if u1 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2370`]

use crate::{
    error::RuntimeError,
    types::{Cell, UnsignedDoubleCellInteger, UnsignedInteger, cell_pair_to_double_number},
};

crate::define_word!(UMSlashMod, "UM/MOD", |it, _tks| {
    let u1: UnsignedInteger = it.pop_last_stack()? as _;
    let ud: UnsignedDoubleCellInteger = cell_pair_to_double_number(it.pop_pair_last_stack()?);

    if u1 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let u1: UnsignedDoubleCellInteger = u1 as _;
    let u2: UnsignedInteger = (ud % u1) as _;
    let u3: UnsignedInteger = (ud / u1) as _;

    it.stack.push(u2 as Cell);
    it.stack.push(u3 as Cell);

    Ok(())
});
