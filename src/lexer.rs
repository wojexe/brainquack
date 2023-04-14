use std::fmt::Display;

const VALID_CHARS: [char; 8] = ['>', '<', '+', '-', '.', ',', '[', ']'];

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Vec<Instruction>),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Right => write!(f, ">"),
            Instruction::Left => write!(f, "<"),
            Instruction::Increment => write!(f, "+"),
            Instruction::Decrement => write!(f, "-"),
            Instruction::Output => write!(f, "."),
            Instruction::Input => write!(f, ","),
            Instruction::Loop(ins) => Ok({
                write!(f, "[")?;

                for ins in ins {
                    write!(f, "{}", ins)?;
                }

                write!(f, "]")?;
            }),
        }
    }
}

#[derive(Debug)]
pub enum LexerError {
    MissingOpeningBracket(usize),
    MissingEndingBracket,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingEndingBracket => {
                write!(f, "Your code is missing a closing bracket \"]\".")
            }
            Self::MissingOpeningBracket(i) => write!(
                f,
                "Your code is missing an opening bracket for the bracket at index {}",
                i
            ),
        }
    }
}

impl std::error::Error for LexerError {}

fn remove_comments<'a>(input: &'a str) -> impl Iterator<Item = (usize, char)> + 'a {
    input
        .chars()
        .enumerate()
        .filter(|(_, ch)| VALID_CHARS.contains(ch))
}

fn parse_block<I: Iterator<Item = (usize, char)>>(
    iter: &mut I,
    open_brackets: i32,
) -> Result<Vec<Instruction>, LexerError> {
    let mut instructions = vec![];

    loop {
        instructions.push(match iter.next() {
            Some((_, '>')) => Instruction::Right,
            Some((_, '<')) => Instruction::Left,
            Some((_, '+')) => Instruction::Increment,
            Some((_, '-')) => Instruction::Decrement,
            Some((_, '.')) => Instruction::Output,
            Some((_, ',')) => Instruction::Input,
            Some((_, '[')) => Instruction::Loop(parse_block(iter, open_brackets + 1)?),
            Some((i, ']')) => {
                if open_brackets == 0 {
                    return Err(LexerError::MissingOpeningBracket(i));
                }

                return Ok(instructions);
            }
            Some(_) => unreachable!(),
            None => break,
        });
    }

    if open_brackets != 0 {
        return Err(LexerError::MissingEndingBracket);
    }

    Ok(instructions)
}

pub fn parse<S: AsRef<str>>(code: S) -> Result<Vec<Instruction>, LexerError> {
    parse_block(&mut remove_comments(code.as_ref()), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments() {
        assert_eq!(
            remove_comments(
                "+++++[>+df+#$%++[>++>ds+g!d+sg+d>sd+++>+<g<s<<-3]>+>+423>->>+[<]<-]>>.>!-,",
            )
            .map(|(_, c)| c)
            .collect::<String>(),
            "+++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>-,"
        )
    }

    #[test]
    fn parsing_simple() {
        let p = parse("><+-.,[+-]");

        let expected = vec![
            Instruction::Right,
            Instruction::Left,
            Instruction::Increment,
            Instruction::Decrement,
            Instruction::Output,
            Instruction::Input,
            Instruction::Loop(vec![Instruction::Increment, Instruction::Decrement]),
        ];

        assert!(p.is_ok());
        assert_eq!(p.unwrap(), expected);
    }

    #[test]
    fn parsing_hello_world() {
        let p = parse(include_str!("tests/helloWorld.bf"));

        assert!(p.is_ok());
    }
}
