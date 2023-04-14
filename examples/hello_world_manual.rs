use std::num::Wrapping;

use brainquack::*;

fn main() -> Result<(), BrainQuackError> {
    let hello_world_instructions: Vec<Instruction> =
        parse(include_str!("../programs/helloWorld.bf"))?;

    let mut memory = vec![Wrapping(0u8); 30_000];
    let mut output = String::new();

    interpret(&hello_world_instructions, &mut memory, Some(&mut output))?;

    print!("{output}");

    Ok(())
}
