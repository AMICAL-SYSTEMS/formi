//! 6.1.2214 SM/REM
//! s-m-slash-rem CORE
//!
//!     ( d1 n1 -- n2 n3 )
//!
//! Divide d1 by n1, giving the symmetric quotient n3 and the remainder n2.
//! Input and output stack arguments are signed. An ambiguous condition exists
//! if n1 is zero or if the quotient lies outside the range of a single-cell
//! signed integer.
//!
//! AMICAL NOTE: Returns a runtime error if n1 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2214`]

use crate::{
    error::RuntimeError,
    types::{Cell, Number, SignedDoubleCellInteger, cell_pair_to_double_number},
};

crate::define_word!(SMSlashRem, "SM/REM", |it, _tks| {
    let n1: Number = it.pop_last_stack()? as _;
    let d1: SignedDoubleCellInteger = cell_pair_to_double_number(it.pop_pair_last_stack()?) as _;

    if n1 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let n1: SignedDoubleCellInteger = n1 as _;
    let n3: Number = (d1 / n1) as _;
    let n2: Number = (d1 % n1) as _;

    it.stack.push(n2 as Cell);
    it.stack.push(n3 as Cell);

    Ok(())
});
