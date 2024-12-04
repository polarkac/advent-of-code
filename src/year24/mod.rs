use crate::{aoc::Aoc, cli::Args, error::Error};

mod day01;
mod day02;
mod day03;
mod day04;

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
        ),
        _ => Err(Error::InvalidDay),
    }
}
