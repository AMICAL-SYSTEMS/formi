//! 6.1.1810 M*
//! m-star CORE
//!
//!     ( n1 n2 -- d )
//!
//! d is the signed product of n1 times n2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.1810`]

use crate::types::{
    Number, SignedDoubleCellInteger, UnsignedDoubleCellInteger, double_number_to_cell_pair,
};

crate::define_word!(MStar, "M*", |it, _tks| {
    let n2: Number = it.pop_last_stack()? as _;
    let n1: Number = it.pop_last_stack()? as _;

    let d: SignedDoubleCellInteger =
        n1 as SignedDoubleCellInteger * n2 as SignedDoubleCellInteger;
    it.push_pair_stack(double_number_to_cell_pair(
        d as UnsignedDoubleCellInteger,
    ));

    Ok(())
});
