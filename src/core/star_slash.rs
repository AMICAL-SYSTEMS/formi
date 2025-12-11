//! 6.1.0100 */
//! star-slash CORE
//!
//!     ( n1 n2 n3 -- n4 )
//!
//! Multiply n1 by n2 producing the intermediate double-cell result d.
//! Divide d by n3 giving the single-cell quotient n4. An ambiguous condition
//! exists if n3 is zero or if the quotient n4 lies outside the range of a
//! signed number. If d and n3 differ in sign, the implementation-defined result
//! returned will be the same as that returned by either the phrase `>R M* R>
//! FM/MOD SWAP DROP` or the phrase `>R M* R> SM/REM SWAP DROP`.
//!
//! AMICAL NOTE: Returns a runtime error if n3 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0100`]

use crate::{
    error::RuntimeError,
    types::{Cell, Number, SignedDoubleCellInteger},
};

crate::define_word!(StarSlash, "*/", |it, _tks| {
    let n3: Number = it.pop_last_stack()? as _;
    let n2: Number = it.pop_last_stack()? as _;
    let n1: Number = it.pop_last_stack()? as _;

    if n3 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let d: SignedDoubleCellInteger = n1 as SignedDoubleCellInteger * n2 as SignedDoubleCellInteger;
    let n4: Number = d.div_euclid(n3 as SignedDoubleCellInteger) as _;
    it.stack.push(n4 as Cell);

    Ok(())
});
