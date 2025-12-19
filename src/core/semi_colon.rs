use crate::interpreter::InterpreterState;

crate::define_word!(SemiColon, ";", |it, _tks| {
    it.state = InterpreterState::Interpret;
    Ok(())
});
