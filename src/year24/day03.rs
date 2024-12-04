use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 03)
}

pub fn part1(input: &str) -> String {
    let lexer = lexer(input);
    let mut total = 0;
    for s in lexer {
        if let Symbol::Mul(num1, num2) = s {
            total += num1.saturating_mul(num2);
        }
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let lexer = lexer(input);
    let mut total = 0;
    let mut should_do = true;
    for s in lexer {
        match s {
            Symbol::Do => should_do = true,
            Symbol::Dont => should_do = false,
            Symbol::Mul(num1, num2) => {
                if should_do {
                    total += num1.saturating_mul(num2);
                }
            }
        }
    }

    total.to_string()
}

fn handle_mul(scanner: &mut Scanner) -> Option<Symbol> {
    scanner.advance_by(4);
    let mut num1: Vec<i64> = Vec::new();
    let mut num2: Vec<i64> = Vec::new();
    let mut add_to_one = true;
    while !scanner.at_end() {
        let c = scanner.pop().unwrap();
        if c == String::from(",") {
            add_to_one = false;
            continue;
        }
        if c == String::from(")") {
            let num1 =
                num1.iter().rev().enumerate().fold(0, |mut num, (i, n)| {
                    num += n * 10_i64.pow(i as u32);

                    num
                });
            let num2 =
                num2.iter().rev().enumerate().fold(0, |mut num, (i, n)| {
                    num += n * 10_i64.pow(i as u32);

                    num
                });

            return Some(Symbol::Mul(num1, num2));
        }
        match c.parse() {
            Ok(num) => {
                if add_to_one {
                    num1.push(num);
                } else {
                    num2.push(num);
                }
            }
            Err(_) => return None,
        };
    }
    None
}

fn lexer(input: &str) -> Vec<Symbol> {
    let mut scanner = Scanner::new(input);
    let mut lexer = Vec::new();

    while !scanner.at_end() {
        if let Some("mul(") = scanner.peek(4).as_deref() {
            if let Some(mul) = handle_mul(&mut scanner) {
                lexer.push(mul);
            }
            continue;
        }
        if let Some("do()") = scanner.peek(4).as_deref() {
            lexer.push(Symbol::Do);
            scanner.advance_by(4);
            continue;
        }
        if let Some("don't()") = scanner.peek(7).as_deref() {
            lexer.push(Symbol::Dont);
            scanner.advance_by(7);
            continue;
        }
        scanner.advance_by(1);
    }

    lexer
}

#[derive(Debug)]
enum Symbol {
    Mul(i64, i64),
    Do,
    Dont,
}

struct Scanner<'a> {
    cursor: usize,
    chars: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(chars: &'a str) -> Self {
        Self { cursor: 0, chars }
    }

    pub fn peek(&self, len: usize) -> Option<String> {
        self.chars
            .get(self.cursor..(self.cursor + len))
            .map(|v| v.to_string())
    }

    pub fn advance_by(&mut self, by: usize) {
        self.cursor += by;
    }

    pub fn pop(&mut self) -> Option<String> {
        let c = self.chars.get(self.cursor..self.cursor + 1);
        if let Some(_) = c {
            self.cursor += 1;
        }

        c.map(|v| v.to_string())
    }

    pub fn at_end(&self) -> bool {
        self.chars.len() == self.cursor
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_success() {
        let preview_input = "mul(10,5)";
        assert_eq!("50", part1(preview_input));
    }

    #[test]
    fn test_unclosed_bracket() {
        let preview_input = "mul(257,900@{select(66,790)";
        assert_eq!("0", part1(preview_input));
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

    #[test]
    fn test_preview_part2() {
        let preview_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", part2(preview_input));
    }
}
