mod macros;
mod error;
mod cli;
mod aoc;
mod year15;
mod year24;


use crate::{error::Error, cli::Args};

fn main() -> Result<(), Error> {
    let args: Args = argh::from_env();
    let aoc = match args.year.as_u16() {
        2015 => year15::build_aoc(&args),
        2024 => year24::build_aoc(&args),
        _ => return Err(Error::InvalidYear),
    };

    aoc?.run();

    Ok(())
}
