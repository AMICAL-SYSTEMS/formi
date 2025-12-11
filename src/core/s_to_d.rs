//! 6.1.2170 S>D
//! s-to-d CORE
//!
//!     ( n -- d )
//!
//! Convert the number n to the double-cell number d with the same
//! numerical value.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2170`]

use crate::types::{
    Number, SignedDoubleCellInteger, UnsignedDoubleCellInteger, double_number_to_cell_pair,
};

crate::define_word!(SToD, "S>D", |it, _tks| {
    let n: Number = it.pop_last_stack()? as _;

    let d: UnsignedDoubleCellInteger = (n as SignedDoubleCellInteger) as _;
    it.push_pair_stack(double_number_to_cell_pair(d));

    Ok(())
});
