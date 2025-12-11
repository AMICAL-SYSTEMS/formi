//! 6.1.1561 FM/MOD
//! f-m-slash-mod CORE
//!
//!     ( d1 n1 -- n2 n3 )
//!
//! Divide d1 by n1, giving the floored quotient n3 and the remainder n2.
//! Input and output stack arguments are signed. An ambiguous condition exists
//! if n1 is zero or if the quotient lies outside the range of a single-cell
//! signed integer.
//!
//! AMICAL NOTE: Returns a runtime error if n1 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1561`]
use crate::{
    error::RuntimeError,
    types::{Cell, Number, SignedDoubleCellInteger, cell_pair_to_double_number},
};

crate::define_word!(FMSlashMod, "FM/MOD", |it, _tks| {
    let n1: Number = it.pop_last_stack()? as _;
    let d1: SignedDoubleCellInteger = cell_pair_to_double_number(it.pop_pair_last_stack()?) as _;

    if n1 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let n2: Number = {
        let n1: SignedDoubleCellInteger = n1 as _;
        let r = d1 % n1;
        if (r > 0 && n1 < 0) || (r < 0 && n1 > 0) {
            r + n1
        } else {
            r
        }
    } as _;

    let n3: Number = {
        let n1: SignedDoubleCellInteger = n1 as _;
        let (d, r) = (d1 / n1, d1 % n1);
        if (r > 0 && n1 < 0) || (r < 0 && n1 > 0) {
            d - 1
        } else {
            d
        }
    } as _;

    it.stack.push(n2 as Cell);
    it.stack.push(n3 as Cell);

    Ok(())
});
