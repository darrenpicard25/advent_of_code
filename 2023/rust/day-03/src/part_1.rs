use crate::error::AocResult;

enum Character {
    None,
    Symbol(char),
    Number(u32),
}

impl Character {
    fn is_symbol(&self) -> bool {
        if let Self::Symbol(_) = self {
            true
        } else {
            false
        }
    }
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

    let result = numbers
        .into_iter()
        .filter(|num_grouping| {
            num_grouping.into_iter().any(|((x, y), _)| {
                is_symbol(&data, x.saturating_sub(1), *y)
                    || is_symbol(&data, x.saturating_sub(1), y + 1)
                    || is_symbol(&data, *x, y + 1)
                    || is_symbol(&data, x + 1, y + 1)
                    || is_symbol(&data, x + 1, *y)
                    || is_symbol(&data, x + 1, y.saturating_sub(1))
                    || is_symbol(&data, *x, y.saturating_sub(1))
                    || is_symbol(&data, x.saturating_sub(1), y.saturating_sub(1))
            })
        })
        .map(|num_group| {
            num_group
                .into_iter()
                .fold(0u32, |acc, ((_, _), num)| acc * 10 + num)
        })
        .sum::<u32>();

    Ok(result.to_string())
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

fn is_symbol(grid: &Vec<Vec<Character>>, x: usize, y: usize) -> bool {
    grid.get(y)
        .map(|row| row.get(x).unwrap_or(&Character::None).is_symbol())
        .unwrap_or(false)
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
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("4361", process(file)?);
        Ok(())
    }
}
