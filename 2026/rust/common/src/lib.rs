pub mod error;

pub use error::{AocError, AocResult};

use std::fmt::Display;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
    #[default]
    Both,
}

impl Part {
    pub fn from_arg(arg: &str) -> AocResult<Self> {
        match arg.trim().to_ascii_lowercase().as_str() {
            "" | "both" | "all" => Ok(Self::Both),
            "1" | "part1" | "part_1" | "part-1" => Ok(Self::One),
            "2" | "part2" | "part_2" | "part-2" => Ok(Self::Two),
            _ => Err(AocError::InvalidArg(arg.to_owned())),
        }
    }

    pub fn from_opt(arg: Option<&str>) -> AocResult<Self> {
        match arg {
            Some(arg) => Self::from_arg(arg),
            None => Ok(Self::Both),
        }
    }
}

impl FromStr for Part {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_arg(s)
    }
}

fn format_duration(d: Duration) -> String {
    let nanos = d.as_nanos();
    if nanos < 1_000 {
        format!("{nanos}ns")
    } else if nanos < 1_000_000 {
        format!("{:.2}µs", nanos as f64 / 1_000.0)
    } else if nanos < 1_000_000_000 {
        format!("{:.2}ms", nanos as f64 / 1_000_000.0)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

fn run_part<T: Display>(
    label: &str,
    input: &str,
    solver: fn(&str) -> AocResult<T>,
) -> AocResult<()> {
    let start = Instant::now();
    let result = solver(input)?;
    let elapsed = start.elapsed();
    println!("{label}: {result} ({})", format_duration(elapsed));
    Ok(())
}

pub fn run_day<T1: Display, T2: Display>(
    input: &str,
    part1: fn(&str) -> AocResult<T1>,
    part2: fn(&str) -> AocResult<T2>,
) -> AocResult<()> {
    let part = Part::from_opt(std::env::args().nth(1).as_deref())?;

    match part {
        Part::One => run_part("Part 1", input, part1)?,
        Part::Two => run_part("Part 2", input, part2)?,
        Part::Both => {
            run_part("Part 1", input, part1)?;
            run_part("Part 2", input, part2)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_part_args() {
        assert_eq!(Part::default(), Part::Both);
        assert_eq!(Part::from_opt(None).unwrap(), Part::Both);
        assert_eq!(Part::from_opt(Some("")).unwrap(), Part::Both);
        assert_eq!(Part::from_arg("").unwrap(), Part::Both);
        assert_eq!(Part::from_arg("   ").unwrap(), Part::Both);
        assert_eq!(Part::from_arg("1").unwrap(), Part::One);
        assert_eq!(Part::from_arg("part1").unwrap(), Part::One);
        assert_eq!(Part::from_arg("part_1").unwrap(), Part::One);
        assert_eq!(Part::from_arg("Part-1").unwrap(), Part::One);
        assert_eq!(Part::from_arg("2").unwrap(), Part::Two);
        assert_eq!(Part::from_arg("part2").unwrap(), Part::Two);
        assert_eq!(Part::from_arg("part_2").unwrap(), Part::Two);
        assert_eq!(Part::from_arg("PART-2").unwrap(), Part::Two);
        assert_eq!(Part::from_arg("both").unwrap(), Part::Both);
        assert_eq!(Part::from_arg("BOTH").unwrap(), Part::Both);
        assert_eq!(Part::from_arg("all").unwrap(), Part::Both);
        assert_eq!("1".parse::<Part>().unwrap(), Part::One);
        assert_eq!("".parse::<Part>().unwrap(), Part::Both);
        assert!(Part::from_arg("3").is_err());
    }
}
