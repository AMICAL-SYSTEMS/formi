//! 6.1.0380 2DUP
//! two-dupe CORE
//!
//!     ( x1 x2 -- x1 x2 x1 x2 )
//!
//! Duplicate cell pair x1 x2.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0380`]

use crate::types::CellPair;

crate::define_word!(TwoDupe, "2DUP", |it, _tks| {
    let x1_x2: CellPair = it.pop_pair_last_stack()?;

    it.push_pair_stack(x1_x2);
    it.push_pair_stack(x1_x2);

    Ok(())
});
