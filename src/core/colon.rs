use alloc::{string::ToString, vec};

use crate::{core::FIXED_TOKENS_MAP, error::RuntimeError, interpreter::InterpreterState};

crate::define_word!(Colon, ":", |it, tks| {
    it.state = InterpreterState::Word;
    let word_token = tks.next().ok_or(RuntimeError::EmptyStack)?;
    // Not a known word, not a number
    let is_word = !FIXED_TOKENS_MAP.contains_key(word_token) && word_token.parse::<u64>().is_err();

    if is_word {
        let word = word_token;

        let mut def_token = vec![];
        for token in tks {
            match token {
                ";" => break,
                other => def_token.push(other),
            }
        }

        if it
            .definitions
            .insert(word.to_string(), def_token.join(" "))
            .is_some()
        {
            return Err(RuntimeError::ExpectedWord);
        }
    } else {
        return Err(RuntimeError::ExpectedWord);
    }
    Ok(())
});
