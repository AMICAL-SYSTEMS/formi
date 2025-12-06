//! 6.1.0690 ABS
//! abs CORE
//!
//!     ( n -- u )
//!
//! u is the absolute value of n.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0690`]

use crate::types::{Number, UnsignedInteger};
crate::define_word!(Abs, "ABS", |it, _tks| {
    let n: Number = it.pop_last_stack()? as Number;
    let u: UnsignedInteger = n.unsigned_abs();
    it.stack.push(u);

    Ok(())
});
