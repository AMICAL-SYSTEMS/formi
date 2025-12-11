//! 6.1.0430 2SWAP
//! two-swap CORE
//!
//!     ( x1 x2 x3 x4 -- x3 x4 x1 x2 )
//!
//! Exchange the top two cell pairs.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0430`]

use crate::types::CellPair;

crate::define_word!(TwoSwap, "2SWAP", |it, _tks| {
    let x3_x4: CellPair = it.pop_pair_last_stack()?;
    let x1_x2: CellPair = it.pop_pair_last_stack()?;

    it.push_pair_stack(x3_x4);
    it.push_pair_stack(x1_x2);

    Ok(())
});
