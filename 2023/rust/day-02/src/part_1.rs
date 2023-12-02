use crate::error::AocResult;

#[derive(Debug)]
enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

#[derive(PartialEq, Debug)]
struct Round {
    green: usize,
    blue: usize,
    red: usize,
}

impl Round {
    fn new() -> Self {
        Self {
            blue: 0,
            green: 0,
            red: 0,
        }
    }
    fn update(&mut self, color: Color) {
        match color {
            Color::Red(value) => self.red += value,
            Color::Green(value) => self.green += value,
            Color::Blue(value) => self.blue += value,
        }
    }

    fn can_contain(&self, compare_results: &Round) -> bool {
        self.green >= compare_results.green
            && self.red >= compare_results.red
            && self.blue >= compare_results.blue
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let color = value
            .trim()
            .chars()
            .filter(|char| char.is_alphabetic())
            .collect::<String>();

        let number = value
            .trim()
            .chars()
            .filter(|char| char.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .map_err(|e| e.to_string())?;

        match color.as_str() {
            "red" => Ok(Color::Red(number)),
            "blue" => Ok(Color::Blue(number)),
            "green" => Ok(Color::Green(number)),
            unknown => Err(format!("Unknown color from string: {unknown}")),
        }
    }
}

fn parse(line: &str) -> (usize, Vec<Round>) {
    let mut x = line.split(':');

    let game_number = x
        .next()
        .map(|contents| {
            contents
                .chars()
                .filter(|char| char.is_digit(10))
                .collect::<String>()
                .parse::<usize>()
                .expect("No game number")
        })
        .unwrap();

    let round_results = x
        .next()
        .map(|contents| {
            contents.split(';').map(|round| {
                let round_result = round
                    .split(',')
                    .fold(Round::new(), |mut results, color_str| {
                        let color = Color::try_from(color_str).unwrap();
                        results.update(color);

                        results
                    });

                round_result
            })
        })
        .unwrap()
        .collect::<Vec<Round>>();

    (game_number, round_results)
}

pub fn process(input: &str) -> AocResult<String> {
    let main_game = Round {
        blue: 14,
        green: 13,
        red: 12,
    };

    let results = input
        .lines()
        .filter_map(|line| {
            let (game_num, round_results) = parse(line);

            if round_results
                .iter()
                .all(|round| main_game.can_contain(round))
            {
                Some(game_num)
            } else {
                None
            }
        })
        .sum::<usize>();

    Ok(results.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("8", process(file)?);
        Ok(())
    }
}
