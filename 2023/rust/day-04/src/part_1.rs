use std::collections::HashSet;

use crate::error::AocResult;

pub fn process(input: &str) -> AocResult<String> {
    Ok(input
        .lines()
        .map(|line| line.split(':').last().expect("Expect data after :"))
        .map(|line| {
            let mut cards = line.split('|');
            let winning_nums = cards
                .next()
                .expect("Missing winning numbers")
                .split_ascii_whitespace()
                .map(|num| {
                    num.parse::<u32>()
                        .expect("Unable to parse string to number")
                })
                .collect::<HashSet<u32>>();

            let guesses = cards
                .next()
                .expect("Missing guessed numbers")
                .split_ascii_whitespace()
                .map(|num| {
                    num.parse::<u32>()
                        .expect("Unable to parse string to number")
                });

            guesses
                .filter(move |guess| winning_nums.contains(guess))
                .fold(0_u32, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("13", process(file)?);
        Ok(())
    }
}
