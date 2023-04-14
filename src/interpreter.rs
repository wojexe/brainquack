use crate::lexer::Instruction;
use std::num::Wrapping;
use std::{fmt::Write, io::Read};

fn interpret_internal(
    instructions: &Vec<Instruction>,
    memory: &mut Vec<Wrapping<u8>>,
    output: &mut String,
    ptr: &mut usize,
) -> Result<(), Box<dyn std::error::Error>> {
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
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(output) = output {
        interpret_internal(instructions, memory, output, &mut 0)
    } else {
        let mut proxied_output = String::new();
        let res = interpret_internal(instructions, memory, &mut proxied_output, &mut 0);

        println!("{}", proxied_output);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::parse;

    #[test]
    fn hello_world() {
        let instructions = parse(include_str!("tests/helloWorld.bf")).unwrap();
        let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 30_000];
        let mut output = String::new();

        assert!(interpret(&instructions, &mut memory, Some(&mut output)).is_ok());
        assert_eq!(output, "\0Hello World!\n".to_string());
    }

    #[test]
    fn cell_size() {
        let instructions = parse(include_str!("tests/cellSize.bf")).unwrap();
        let mut memory: Vec<Wrapping<u8>> = vec![Wrapping(0); 30_000];
        let mut output = String::new();

        assert!(interpret(&instructions, &mut memory, Some(&mut output)).is_ok());
        assert_eq!(output, "\08 bit cells\n".to_string());
    }
}
