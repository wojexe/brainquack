use brainquack::{execute, BrainQuackError};

fn main() -> Result<(), BrainQuackError> {
    execute(include_str!("../programs/head.bf"), None)
}
