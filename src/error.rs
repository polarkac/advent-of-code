use crate::cli::{Day, Year};

#[derive(Debug)]
pub enum Error {
    ParseYear,
    InvalidYear,
    ParseDay,
    InvalidDay,
    InvalidDate,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseYear => {
                write!(
                    f,
                    "Year must be a number in range {} - {}.",
                    Year::MIN,
                    Year::MAX
                )
            }
            Self::InvalidYear => {
                write!(
                    f,
                    "Year must be in range {} - {}.",
                    Year::MIN,
                    Year::MAX
                )
            }
            Self::ParseDay => {
                write!(
                    f,
                    "Day must be a number in range {} - {}.",
                    Day::MIN,
                    Day::MAX
                )
            }
            Self::InvalidDay => {
                write!(f, "Day must be in range {} - {}.", Day::MIN, Day::MAX)
            }
            Self::InvalidDate => write!(f, "Invalid date"),
        }
    }
}

impl std::error::Error for Error {}
