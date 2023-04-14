use brainquack::*;

fn main() -> Result<(), BrainQuackError> {
    let hello_world = lexer::parse(include_str!("../programs/helloWorld.bf"))?;
    let cell_size = lexer::parse(include_str!("../programs/cellSize.bf"))?;

    let mut memory = vec![std::num::Wrapping(0); 30_000];

    if let Err(e) = interpreter::interpret(&hello_world, &mut memory, None) {
        println!("{e}");
    }

    memory.fill(std::num::Wrapping(0));

    if let Err(e) = interpreter::interpret(&cell_size, &mut memory, None) {
        println!("{e}");
    }

    Ok(())
}
