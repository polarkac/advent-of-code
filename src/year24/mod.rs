use crate::{aoc::Aoc, cli::Args, error::Error};

pub mod day01;

pub fn build_aoc(args: &Args) -> Result<Aoc<'_>, Error> {
    match args.day.as_u8() {
        1 => Ok(
            Aoc::new(args, Box::new(day01::input))
                .part(Box::new(day01::part1))
                .part(Box::new(day01::part2)),
        ),
        _ => Err(Error::InvalidDay),
    }
}
