//! 6.1.0110 */MOD
//! star-slash-mod CORE
//!
//!     ( n1 n2 n3 -- n4 n5 )
//!
//! Multiply n1 by n2 producing the intermediate double-cell result d. Divide d
//! by n3 producing the single-cell remainder n4 and the single-cell quotient
//! n5. An ambiguous condition exists if n3 is zero, or if the quotient n5 lies
//! outside the range of a single-cell signed integer. If d and n3 differ in
//! sign, the implementation-defined result returned will be the same as that
//! returned by either the phrase `>R M* R> FM/MOD` or the phrase `>R M* R>
//! SM/REM`.
//!
//! AMICAL NOTE: Returns a runtime error if n3 is zero.
//!
//! [`https://www.taygeta.com/forth/dpans6.htm#6.1.0110`]

use crate::{
    error::RuntimeError,
    types::{Cell, Number, SignedDoubleCellInteger},
};

crate::define_word!(StarSlashMod, "*/MOD", |it, _tks| {
    let n3: Number = it.pop_last_stack()? as Number;
    let n2: Number = it.pop_last_stack()? as Number;
    let n1: Number = it.pop_last_stack()? as Number;

    if n3 == 0 {
        return Err(RuntimeError::DivisionByZero);
    }

    let d: SignedDoubleCellInteger = n1 as SignedDoubleCellInteger * n2 as SignedDoubleCellInteger;

    let n3: SignedDoubleCellInteger = n3 as _;
    let n5: Number = d.div_euclid(n3) as _;
    let n4: Number = d.rem_euclid(n3) as _;

    // TODO: Warning if n5 overflows?
    
    it.stack.push(n4 as Cell);
    it.stack.push(n5 as Cell);

    Ok(())
});
