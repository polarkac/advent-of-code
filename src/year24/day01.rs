use std::collections::HashMap;

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 01)
}

pub fn part1(input: &str) -> String {
    let pairs = as_pairs(input);
    let mut total_distance = 0;
    for i in 0..pairs.0.len() {
        let pair = (pairs.0[i], pairs.1[i]);
        total_distance += pair.0.abs_diff(pair.1);
    }

    total_distance.to_string()
}

pub fn part2(input: &str) -> String {
    let (l_col, r_col) = as_pairs(input);
    let map = r_col.iter().fold(HashMap::new(), |mut map, num| {
        if map.contains_key(num) {
            map.entry(*num).and_modify(|n| *n += num) ;
        } else {
            map.insert(*num, *num);
        }
        map
    });

    let mut similarity_score = 0;
    for num in l_col {
        if let Some(value) = map.get(&num) {
            similarity_score += value;
        }
    }

    similarity_score.to_string()
}

fn as_pairs(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut col1, mut col2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().split_once(" "))
        .filter(|pair| pair.is_some())
        .map(|pair| {
            let pair = pair.unwrap();
            let num1: u32 = pair.0.trim().parse().unwrap();
            let num2: u32 = pair.1.trim().parse().unwrap();

            (num1, num2)
        })
        .unzip();
    col1.sort();
    col2.sort();

    (col1, col2)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    #[test]
    fn test_preview_part1() {
        let preview_input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ";

        let result = part1(preview_input);
        assert_eq!(String::from("11"), result);
    }

    #[test]
    fn test_preview_part2() {
        let preview_input = "
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        ";

        let result = part2(preview_input);
        assert_eq!(String::from("31"), result);
    }
}
