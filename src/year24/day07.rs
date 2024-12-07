use std::str::FromStr;

use itertools::Itertools;

use crate::include_input;

pub fn input() -> &'static str {
    include_input!(2024 / 07)
}

pub fn part1(input: &str) -> String {
    let calibrations = Calibrations::from_str(input).unwrap();
    let mut total = 0;
    for equation in &calibrations.equations {
        total += equation.try_solutions();
    }

    total.to_string()
}

pub fn part2(input: &str) -> String {
    let calibrations = Calibrations::from_str(input).unwrap();
    let mut total = 0;
    for equation in &calibrations.equations {
        total += equation.try_solutions_with_concat();
    }

    total.to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mul,
    Con,
}

impl Operator {
    fn combinations_without_concat(size: usize) -> Vec<Vec<Self>> {
        (0..size)
            .map(|_| [Self::Add, Self::Mul].into_iter())
            .multi_cartesian_product()
            .collect_vec()
    }

    fn combinations_with_concat(size: usize) -> Vec<Vec<Self>> {
        (0..size)
            .map(|_| [Self::Add, Self::Mul, Self::Con].into_iter())
            .multi_cartesian_product()
            .collect_vec()
    }

    fn solve(&self, left: u64, right: u64) -> u64 {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
            Self::Con => {
                let len = ((right as f64).log10() + 1.0).floor() as u32;
                left * 10_u64.pow(len) + right
            }
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: u64,
    values: Vec<u64>,
}

impl Equation {
    fn try_solutions(&self) -> usize {
        let op_combinations = Operator::combinations_without_concat(self.values.len() - 1);

        self.solutions(op_combinations)
    }

    fn try_solutions_with_concat(&self) -> usize {
        let op_combinations = Operator::combinations_with_concat(self.values.len() - 1);

        self.solutions(op_combinations)
    }

    fn solutions(&self, op_combinations: Vec<Vec<Operator>>) -> usize {
        for operators in op_combinations {
            let mut iter = self.values.iter();
            let mut left = *iter.next().unwrap();
            let mut right = *iter.next().unwrap();
            for operator in operators {
                left = operator.solve(left, right);
                match iter.next() {
                    Some(v) => right = *v,
                    None => break,
                }
            }
            if left == self.result {
                return self.result as usize;
            }
        }

        0
    }
}

#[derive(Debug)]
struct Calibrations {
    equations: Vec<Equation>,
}

impl FromStr for Calibrations {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations: Vec<Equation> = s
            .lines()
            .filter_map(|line| match line.split_once(":") {
                Some((result, values)) => {
                    let result = result.trim().parse().ok()?;
                    let values = values
                        .split(" ")
                        .filter_map(|value| value.trim().parse().ok())
                        .collect();

                    Some(Equation { result, values })
                }
                None => return None,
            })
            .collect();

        Ok(Self { equations })
    }
}

#[cfg(test)]
mod test {
    use crate::year24::day07::{part1, part2, Operator};

    #[test]
    fn test_preview_part1() {
        let preview_input = "190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20";
        assert_eq!("3749", part1(preview_input));
    }

    #[test]
    fn test_preview_part2() {
        let preview_input = "190: 10 19
            3267: 81 40 27
            83: 17 5
            156: 15 6
            7290: 6 8 6 15
            161011: 16 10 13
            192: 17 8 14
            21037: 9 7 18 13
            292: 11 6 16 20";
        assert_eq!("11387", part2(preview_input));
    }

    #[test]
    fn test_operator_combinations() {
        assert_eq!(
            vec![vec![Operator::Add], vec![Operator::Mul]],
            Operator::combinations_without_concat(1)
        );
        assert_eq!(
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Mul],
                vec![Operator::Mul, Operator::Add],
                vec![Operator::Mul, Operator::Mul]
            ],
            Operator::combinations_without_concat(2)
        );
    }
}
