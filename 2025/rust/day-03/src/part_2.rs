use crate::error::AocResult;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn process(input: &str) -> AocResult<String> {
    let answer: usize = input
        .lines()
        .par_bridge()
        .map(|line| {
            println!("Processing line: {line}");

            line.chars()
                .combinations(12)
                .map(|combination| {
                    combination
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .inspect(|answer| {
            dbg!(answer);
        })
        .sum();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("3121910778619", process(file)?);
        Ok(())
    }
}
