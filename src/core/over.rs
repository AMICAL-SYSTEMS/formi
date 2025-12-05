//! 6.1.1990 OVER
//! CORE
//! 
//!     ( x1 x2 -- x1 x2 x1 )
//! 
//! Place a copy of x1 on top of the stack
//! 
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.1990]
crate::define_word!(Over, "OVER", |it, _tks| {
    let x2 = it.pop_last_stack()?;
    let x1 = it.pop_last_stack()?;

    it.stack.push(x1);
    it.stack.push(x2);
    it.stack.push(x1);

    Ok(())
});