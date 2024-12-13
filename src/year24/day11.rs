use std::collections::HashMap;

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 11)
}

pub fn part1(input: &str) -> String {
    let stones = parse_input(input);

    stone_blinks(stones, 25).to_string()
}

pub fn part2(input: &str) -> String {
    let stones = parse_input(input);

    stone_blinks(stones, 75).to_string()
}

fn stone_blinks(mut stones: HashMap<Stone, u64>, blinks: u8) -> u64 {
    for _ in 0..blinks {
        let mut new_stones = HashMap::new();
        for (stone, counter) in &stones {
            let ruled_stones = stone.apply_rules();
            for ruled_stone in ruled_stones {
                new_stones
                    .entry(ruled_stone)
                    .and_modify(|c| *c += 1 * counter)
                    .or_insert(1 * counter);
            }
        }
        stones = new_stones;
    }

    stones.into_iter().fold(0_u64, |mut acc, (_, counter)| {
        acc += counter;
        acc
    })
}

fn parse_input(input: &str) -> HashMap<Stone, u64> {
    input.split(" ").fold(HashMap::new(), |mut acc, value| {
        let number = Stone(value.trim().parse().unwrap());
        acc.entry(number)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        acc
    })
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct Stone(u64);

impl Stone {
    fn apply_rules(&self) -> Vec<Self> {
        let default_value = self.0 * 2024;
        if self.0 == 0 {
            return vec![Stone(1)];
        }
        if let Some(split) = self.try_split() {
            return split;
        }

        vec![Stone(default_value)]
    }

    fn try_split(&self) -> Option<Vec<Self>> {
        let digit_count = ((self.0 as f64).log10().floor() + 1.0) as u64;
        if digit_count % 2 != 0 {
            return None;
        }
        let half = digit_count / 2;
        let power = 10_u64.pow(half as u32);
        let first_number = self.0 / power;
        let second_number = self.0 - (first_number * power);
        let first_stone = Stone(first_number);
        let second_stone = Stone(second_number);

        Some(vec![first_stone, second_stone])
    }
}

#[cfg(test)]
mod test {
    use crate::year24::day11::stone_blinks;

    use super::{parse_input, Stone};

    const PREVIEW_INPUT: &str = "125 17";

    #[test]
    fn test_preview_part1() {
        let stones = parse_input(PREVIEW_INPUT);
        assert_eq!(55312, stone_blinks(stones, 25));
    }

    #[test]
    fn test_stone_rules() {
        let stone = Stone(0);
        assert_eq!(vec![Stone(1)], stone.apply_rules());
    }

    #[test]
    fn test_stone_split() {
        let stone = Stone(3456);
        assert_eq!(vec![Stone(34), Stone(56)], stone.apply_rules());
        let stone = Stone(90284098);
        assert_eq!(vec![Stone(9028), Stone(4098)], stone.apply_rules());
    }

    #[test]
    fn test_stone_default() {
        let stone = Stone(123);
        assert_eq!(vec![Stone(248952)], stone.apply_rules());
    }
}
