use std::collections::{HashMap, HashSet};

use crate::error::AocResult;

pub fn process(input: &str) -> AocResult<String> {
    let mut game_tracker: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let mut game_details = line.split(':');
        let game_num = game_details
            .next()
            .expect("expect game details")
            .chars()
            .filter(|char| char.is_ascii_digit())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        let game_cards = game_details.next().expect("Expect data after :");

        let mut cards = game_cards.split('|');
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

        game_tracker
            .entry(game_num)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        let num_times_to_run = *game_tracker.get(&game_num).unwrap();

        let num_wins = guesses
            .filter(move |guess| winning_nums.contains(guess))
            .count();

        for win in 1..num_wins + 1 {
            game_tracker
                .entry(game_num + win as u32)
                .and_modify(|e| *e += num_times_to_run)
                .or_insert(num_times_to_run);
        }
    }

    Ok(game_tracker.values().sum::<u32>().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("30", process(file)?);
        Ok(())
    }
}
