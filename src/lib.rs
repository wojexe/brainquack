pub mod interpreter;
pub mod lexer;

pub use interpreter::*;
pub use lexer::*;

#[derive(Debug)]
pub enum BrainQuackError {
    LexerError(LexerError),
    InterpreterError(InterpreterError),
}

impl std::error::Error for BrainQuackError {}

impl std::fmt::Display for BrainQuackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainQuackError::LexerError(e) => write!(f, "{e}"),
            BrainQuackError::InterpreterError(e) => write!(f, "{e}"),
        }
    }
}

impl From<LexerError> for BrainQuackError {
    fn from(value: LexerError) -> Self {
        BrainQuackError::LexerError(value)
    }
}

impl From<InterpreterError> for BrainQuackError {
    fn from(value: InterpreterError) -> Self {
        BrainQuackError::InterpreterError(value)
    }
}

pub fn execute<S>(code: S, output: Option<&mut String>) -> Result<(), BrainQuackError>
where
    S: AsRef<str>,
{
    let mut instructions = lexer::parse(code)?;

    interpreter::interpret(
        &mut instructions,
        &mut vec![std::num::Wrapping(0u8); 30_000],
        output,
    )?;

    Ok(())
}
