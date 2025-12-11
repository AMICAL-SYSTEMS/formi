//! 6.1.0370 2DROP
//! two-drop CORE
//!
//!     ( x1 x2 -- )
//!
//! Drop cell pair x1 x2 from the stack.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0370`]

use crate::types::CellPair;

crate::define_word!(TwoDrop, "2DROP", |it, _tks| {
    let _x1_x2: CellPair = it.pop_pair_last_stack()?;

    Ok(())
});
