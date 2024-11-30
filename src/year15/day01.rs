use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2015 / 01)
}

pub fn part1(input: &str) -> String {
    let mut floor = 0;
    let mut basement_char = None;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if floor == -1 && basement_char.is_none() {
            basement_char = Some(i);
        }
    }

    format!("Result: {}", floor)
}
