//! 6.1.2260 SWAP
//! CORE
//! 
//!     ( x1 x2 -- x2 x1 )
//! 
//! Exchange the top two stack items. 
//! 
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.2260]
crate::define_word!(Swap, "SWAP", |it, _tks| {
    let x2 = it.pop_last_stack()?;
    let x1 = it.pop_last_stack()?;

    it.stack.push(x1);
    it.stack.push(x2);

    Ok(())
});