use crate::error::AocResult;

enum Character {
    None,
    Symbol(char),
    Number(u32),
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::None,
            c if c.is_digit(10) => Self::Number(c.to_digit(10).unwrap()),
            c => Self::Symbol(c),
        }
    }
}

pub fn process(input: &str) -> AocResult<String> {
    let data = parse(input);

    let numbers = find_numbers(&data);

    let gears = find_gears(&data);

    let results = gears
        .into_iter()
        .filter_map(|(gear_x, gear_y)| {
            let nums = numbers.iter().filter(|num| {
                num.iter().any(|((x, y), _)| {
                    (gear_x.saturating_sub(1) == *x && gear_y + 1 == *y)
                        || (gear_x.saturating_sub(1) == *x && gear_y == *y)
                        || (gear_x.saturating_sub(1) == *x && gear_y.saturating_sub(1) == *y)
                        || (gear_x == *x && gear_y + 1 == *y)
                        || (gear_x == *x && gear_y.saturating_sub(1) == *y)
                        || (gear_x + 1 == *x && gear_y + 1 == *y)
                        || (gear_x + 1 == *x && gear_y == *y)
                        || (gear_x + 1 == *x && gear_y.saturating_sub(1) == *y)
                })
            });

            if nums.clone().count() != 2 {
                None
            } else {
                Some(
                    nums.map(|num_parts| {
                        num_parts
                            .iter()
                            .fold(0u32, |acc, ((_, _), num)| acc * 10 + num)
                    })
                    .fold(1_u32, |acc, num| acc * num),
                )
            }
        })
        .sum::<u32>();

    Ok(results.to_string())
}

fn find_gears(data: &Vec<Vec<Character>>) -> Vec<(usize, usize)> {
    data.iter()
        .enumerate()
        .map(|(y_index, line)| {
            line.iter()
                .enumerate()
                .fold(
                    Vec::new(),
                    |mut gears, (x_index, character)| match character {
                        Character::Symbol(sym) if *sym == '*' => {
                            gears.push((x_index, y_index));
                            gears
                        }
                        _ => gears,
                    },
                )
        })
        .flatten()
        .collect()
}

fn find_numbers(data: &Vec<Vec<Character>>) -> Vec<Vec<((usize, usize), u32)>> {
    data.iter()
        .enumerate()
        .map(|(y_index, line)| {
            line.iter().enumerate().fold(
                Vec::new(),
                |mut num_tracker: Vec<Vec<((usize, usize), u32)>>, (x_index, character)| {
                    let Character::Number(num) = character else {
                        return num_tracker;
                    };

                    match num_tracker.iter().last() {
                        Some(last_vec) => {
                            match last_vec.iter().last() {
                                Some(((x_coordinate, y_coordinate), _)) => {
                                    if *x_coordinate == x_index.saturating_sub(1)
                                        && *y_coordinate == y_index
                                    {
                                        let last = num_tracker.iter_mut().last().unwrap();
                                        last.push(((x_index, y_index), *num))
                                    } else {
                                        num_tracker.push(vec![((x_index, y_index), *num)])
                                    }
                                }
                                None => panic!("technically impossible"),
                            };
                        }
                        None => num_tracker.push(vec![((x_index, y_index), *num)]),
                    }

                    num_tracker
                },
            )
        })
        .flatten()
        .collect()
}

fn parse(input: &str) -> Vec<Vec<Character>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.into())
                .collect::<Vec<Character>>()
        })
        .collect::<Vec<Vec<Character>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("467835", process(file)?);
        Ok(())
    }
}
