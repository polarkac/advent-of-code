use crate::{aoc::Aoc, cli::Args, error::Error};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn build_aoc(args: &Args) -> Result<Aoc<'_>, Error> {
    match args.day.as_u8() {
        1 => Ok(
            Aoc::new(args, Box::new(day01::input))
                .part(Box::new(day01::part1))
                .part(Box::new(day01::part2)),
        ),
        2 => Ok(
            Aoc::new(args, Box::new(day02::input))
                .part(Box::new(day02::part1))
                .part(Box::new(day02::part2))
        ),
        3 => Ok(
            Aoc::new(args, Box::new(day03::input))
                .part(Box::new(day03::part1))
                .part(Box::new(day03::part2))
        ),
        4 => Ok(
            Aoc::new(args, Box::new(day04::input))
                .part(Box::new(day04::part1))
                .part(Box::new(day04::part2))
        ),
        5 => Ok(
            Aoc::new(args, Box::new(day05::input))
                .part(Box::new(day05::part1))
                .part(Box::new(day05::part2))
        ),
        6 => Ok(
            Aoc::new(args, Box::new(day06::input))
                .part(Box::new(day06::part1))
        ),
        7 => Ok(
            Aoc::new(args, Box::new(day07::input))
                .part(Box::new(day07::part1))
                .part(Box::new(day07::part2))
        ),
        8 => Ok(
            Aoc::new(args, Box::new(day08::input))
                .part(Box::new(day08::part1))
        ),
        _ => Err(Error::InvalidDay),
    }
}
