use crate::interpreter::InterpreterState;

crate::define_word!(SemiColon, ";", |it, _tks| {
    debug_assert_eq!(it.state, InterpreterState::Word);
    it.state = InterpreterState::Normal;
    Ok(())
});
