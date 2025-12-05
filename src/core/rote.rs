//! 6.1.2160 ROT
//! rote CORE
//! 
//!     ( x1 x2 x3 -- x2 x3 x1 )
//!
//! Rotate the top three stack entries. 
//! 
//! [https://www.taygeta.com/forth/dpans6.htm#6.1.2160]
crate::define_word!(Rote, "ROT", |it, _tks| {
    let x3 = it.pop_last_stack()?;
    let x2 = it.pop_last_stack()?;
    let x1 = it.pop_last_stack()?;

    it.stack.push(x2);
    it.stack.push(x3);
    it.stack.push(x1);

    Ok(())
});