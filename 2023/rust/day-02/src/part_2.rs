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
    let results = input
        .lines()
        .map(|line| {
            let (_, round_results) = parse(line);
            let mut results = Round::new();

            round_results.into_iter().for_each(|round| {
                results.blue = usize::max(results.blue, round.blue);
                results.red = usize::max(results.red, round.red);
                results.green = usize::max(results.green, round.green);
            });

            results.blue * results.green * results.red
        })
        .sum::<usize>();

    Ok(results.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("2286", process(file)?);
        Ok(())
    }

    #[test]
    fn test_game_1() -> AocResult<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!("48", process(input)?);
        Ok(())
    }

    #[test]
    fn test_game_2() -> AocResult<()> {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!("12", process(input)?);
        Ok(())
    }
    #[test]
    fn test_game_3() -> AocResult<()> {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!("1560", process(input)?);
        Ok(())
    }

    #[test]
    fn test_game_4() -> AocResult<()> {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!("630", process(input)?);
        Ok(())
    }

    #[test]
    fn test_game_5() -> AocResult<()> {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
