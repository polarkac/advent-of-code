use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 03)
}

pub fn part1(input: &str) -> String {
    let mut parser = Parser::new();
    let mut nums = Vec::new();
    for c in input.chars() {
        parser.transition(c);
        if let ParsingState::Final = parser.state {
            nums.push(parser.multiply());
        }
    }

    nums.iter().sum::<i32>().to_string()
}

pub fn part2(input: &str) -> String {
    String::new()
}

#[derive(Debug)]
enum ParsingState {
    Instruction,
    FirstNum,
    SecondNum,
    Final,
}

#[derive(Debug)]
struct Parser {
    command: String,
    num1: String,
    num2: String,
    state: ParsingState,
}

impl Parser {
    const ALLOWED_CHARS: [char; 16] = [
        'm', 'u', 'l', '(', ')', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '0', ',',
    ];
    pub fn new() -> Self {
        Self {
            command: String::new(),
            num1: String::new(),
            num2: String::new(),
            state: ParsingState::Instruction,
        }
    }

    pub fn transition(&mut self, c: char) {
        if !Self::ALLOWED_CHARS.contains(&c) {
            self.reset();
            return;
        }
        let next_state = self.next_state(c);

        if let None = next_state {
            match self.state {
                ParsingState::Instruction => {
                    self.command.push(c);
                    if self.command.len() == 4 {
                        self.command.remove(0);
                    }
                },
                ParsingState::FirstNum => self.num1.push(c),
                ParsingState::SecondNum => self.num2.push(c),
                _ => {}
            }
        } else if let Some(ParsingState::FirstNum) = next_state {
            self.state = next_state.unwrap();
            if self.command != "mul" {
                self.reset();
            }
        } else {
            self.state = next_state.unwrap();
        }
    }

    pub fn multiply(&mut self) -> i32 {
        let num1: i32 = self.num1.parse().unwrap();
        let num2: i32 = self.num2.parse().unwrap();
        self.reset();

        num1.saturating_mul(num2)
    }

    pub fn reset(&mut self) {
        self.state = ParsingState::Instruction;
        self.command.clear();
        self.num1.clear();
        self.num2.clear();
    }

    fn next_state(&self, c: char) -> Option<ParsingState> {
        match (&self.state, c) {
            (ParsingState::Instruction, '(') => Some(ParsingState::FirstNum),
            (ParsingState::FirstNum, ',') => Some(ParsingState::SecondNum),
            (ParsingState::SecondNum, ')') => Some(ParsingState::Final),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_success() {
        let preview_input = "mul(10,5)";
        assert_eq!("50", part1(preview_input));
    }

    #[test]
    fn test_my_input() {
        let preview_input = "&)lkajsoeriu()mul(10,5)";
        assert_eq!("50", part1(preview_input));
    }

    #[test]
    fn test_preview_part1() {
        let preview_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", part1(preview_input));
    }
}
