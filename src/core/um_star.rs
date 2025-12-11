//! 6.1.2360 UM*
//! u-m-star CORE
//!
//!     ( u1 u2 -- ud )
//!
//! Multiply u1 by u2, giving the unsigned double-cell product ud. All
//! values and arithmetic are unsigned.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.2360`]

use crate::types::{UnsignedDoubleCellInteger, UnsignedInteger, double_number_to_cell_pair};

crate::define_word!(UMStar, "UM*", |it, _tks| {
    let u2: UnsignedInteger = it.pop_last_stack()? as _;
    let u1: UnsignedInteger = it.pop_last_stack()? as _;

    let ud: UnsignedDoubleCellInteger =
        u1 as UnsignedDoubleCellInteger * u2 as UnsignedDoubleCellInteger;

    it.push_pair_stack(double_number_to_cell_pair(ud));

    Ok(())
});
