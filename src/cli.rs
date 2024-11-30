use std::str::FromStr;

use argh::FromArgs;

use crate::error::Error;

pub struct Year(u16);

impl Year {
    pub const MIN: u16 = 2015;
    pub const MAX: u16 = 2024;

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl FromStr for Year {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year: u16 = s.parse().map_err(|_| Error::ParseYear)?;
        if !(Self::MIN..=Self::MAX).contains(&year) {
            return Err(Error::InvalidYear);
        }

        Ok(Year(year))
    }
}

pub struct Day(u8);

impl Day {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 25;

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl FromStr for Day {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: u8 = s.parse().map_err(|_| Error::ParseDay)?;
        if !(Self::MIN..=Self::MAX).contains(&day) {
            return Err(Error::InvalidDay);
        }

        Ok(Day(day))
    }
}

#[derive(FromArgs)]
/// Select what year and day to run for Advent of Code
pub struct Args {
    #[argh(positional)]
    pub year: Year,

    #[argh(positional)]
    pub day: Day,

    /// run a benchmark
    #[argh(switch)]
    pub bench: bool,
}

/*
impl Args {
    pub fn run(&self) -> Result<(), Error> {
        if !self.bench {
            self.run_year()?;
        } else {
            let mut times = Vec::with_capacity(25);
            while times.len() < 25 {
                let instant = Instant::now();
                self.run_year()?;
                times.push((Instant::now() - instant).as_secs_f64());
            }
            for time in times {
                println!("Time: {} μs", time * 1_000_000.0);
            }
        }

        Ok(())
    }

    fn run_year(&self) -> Result<(), Error> {
        match self.year.as_u16() {
            2015 => year15::run_day(&self.day),
            year => return Err(Error::YearNotStarted(year)),
        }
    }
}
*/
