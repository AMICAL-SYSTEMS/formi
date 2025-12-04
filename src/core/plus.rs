pub struct Plus;

impl super::FixedToken for Plus {
    fn execute(
        interpreter: &mut crate::interpreter::Interpreter,
    ) -> Result<(), crate::error::RuntimeError> {
        let a = interpreter.pop_last_stack()?;
        let b = interpreter.pop_last_stack()?;

        interpreter.stack.push(a + b);

        Ok(())
    }
}
