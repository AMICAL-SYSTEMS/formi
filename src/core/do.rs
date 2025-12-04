use crate::interpreter::InterpreterState;
use alloc::vec;

crate::define_word!(Do, "DO", |it, tks| {
    let min_bound = it.pop_last_stack()?;
    let max_bound = it.pop_last_stack()?;
    let range = min_bound..max_bound;

    let mut vec_token = vec![];
    for token in tks {
        match token {
            "LOOP" => break,
            other => vec_token.push(other),
        }
    }

    let looping_tokens = vec_token.join(" ");
    for i in range.clone() {
        it.state = InterpreterState::Loop(range.clone(), i);
        it.execute_tokens(looping_tokens.clone())?;
    }
    it.state = InterpreterState::Normal;

    Ok(())
});
