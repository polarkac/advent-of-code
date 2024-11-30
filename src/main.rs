mod macros;
mod error;
mod cli;
mod aoc;
mod year15;


use crate::{error::Error, cli::Args, aoc::Aoc};

fn main() -> Result<(), Error> {
    let args: Args = argh::from_env();
    let aoc = match (args.year.as_u16(), args.day.as_u8()) {
        (2015, 1) => {
            Aoc::new(Box::new(year15::day01::input))
                .part(Box::new(year15::day01::part1))
        },
        _ => return Err(Error::InvalidDate),
    };

    if !args.bench {
        aoc.run();
    } else {
        aoc.run_benchmarked();
    }

    Ok(())
}
