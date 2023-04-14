use crate::lexer::Instruction;
use std::num::Wrapping;
use std::{fmt::Write, io::Read};

#[derive(Debug)]
pub enum InterpreterError {
    IOError(std::io::Error),
    FMTError(std::fmt::Error),
}

impl std::error::Error for InterpreterError {}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::IOError(e) => write!(f, "{e}"),
            InterpreterError::FMTError(e) => write!(f, "{e}"),
        }
    }
}

impl From<std::io::Error> for InterpreterError {
    fn from(value: std::io::Error) -> Self {
        InterpreterError::IOError(value)
    }
}

impl From<std::fmt::Error> for InterpreterError {
    fn from(value: std::fmt::Error) -> Self {
        InterpreterError::FMTError(value)
    }
}

fn interpret_internal(
    instructions: &Vec<Instruction>,
    memory: &mut Vec<Wrapping<u8>>,
    output: &mut String,
    ptr: &mut usize,
) -> Result<(), InterpreterError> {
    let input = std::io::stdin();

    for instruction in instructions {
        match instruction {
            Instruction::Right => {
                *ptr += 1;

                if *ptr == memory.len() {
                    *ptr = 0
                }
            }
            Instruction::Left => {
                if *ptr == 0 {
                    *ptr = memory.len();
                }

                *ptr -= 1;
            }
            Instruction::Increment => memory[*ptr] += 1,
            Instruction::Decrement => memory[*ptr] -= 1,
            Instruction::Output => write!(output, "{}", memory[*ptr].0 as char)?,
            Instruction::Input => {
                let mut byte = [0u8];
                let handle = input.lock();
                handle.take(1).read_exact(&mut byte)?;
                memory[*ptr] = Wrapping(byte[0]);
            }
            Instruction::Loop(body) => {
                if memory[*ptr].0 != 0 {
                    loop {
                        interpret_internal(body, memory, output, ptr)?;

                        if memory[*ptr].0 == 0 {
                            break;
                        }
                    }
                }
            }
        };
    }

    Ok(())
}

pub fn interpret(
    instructions: &Vec<Instruction>,
    memory: &mut Vec<Wrapping<u8>>,
    output: Option<&mut String>,
) -> Result<(), InterpreterError> {
    if let Some(output) = output {
        interpret_internal(instructions, memory, output, &mut 0)
    } else {
        let mut proxied_output = String::new();
        let res = interpret_internal(instructions, memory, &mut proxied_output, &mut 0);

        print!("{}", proxied_output);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse;

    #[test]
    fn hello_world() {
        let instructions = parse(include_str!("programs/helloWorld.bf")).unwrap();
        let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 30_000];
        let mut output = String::new();

        assert!(interpret(&instructions, &mut memory, Some(&mut output)).is_ok());
        assert_eq!(output, "\0Hello World!\n".to_string());
    }

    #[test]
    fn cell_size() {
        let instructions = parse(include_str!("programs/cellSize.bf")).unwrap();
        let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 30_000];
        let mut output = String::new();

        assert!(interpret(&instructions, &mut memory, Some(&mut output)).is_ok());
        assert_eq!(output, "\08 bit cells\n".to_string());
    }
}
