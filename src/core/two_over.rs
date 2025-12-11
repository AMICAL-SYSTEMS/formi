//! 6.1.0400 2OVER
//! two-over CORE
//!
//!     ( x1 x2 x3 x4 -- x1 x2 x3 x4 x1 x2 )
//!
//! Copy cell pair x1 x2 to the top of the stack.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0400`]

use crate::types::CellPair;

crate::define_word!(TwoOver, "2OVER", |it, _tks| {
    let x3_x4: CellPair = it.pop_pair_last_stack()?;
    let x1_x2: CellPair = it.pop_pair_last_stack()?;

    it.push_pair_stack(x1_x2);
    it.push_pair_stack(x3_x4);
    it.push_pair_stack(x1_x2);

    Ok(())
});
