use std::collections::HashSet;

use advent_2015::Problem;

enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::West,
            '>' => Direction::East,
            '^' => Direction::North,
            'v' => Direction::South,
            _ => panic!("Unknown character attempted to be converted to Direction"),
        }
    }
}

struct Day3Problem;

impl Problem for Day3Problem {
    fn part_one(input: &str) -> String {
        let mut position = (0, 0);
        let mut history = HashSet::new();
        history.insert(position);

        input.chars().for_each(|char| {
            match char.into() {
                Direction::North => {
                    position = (position.0, position.1 + 1);
                }
                Direction::South => {
                    position = (position.0, position.1 - 1);
                }
                Direction::East => {
                    position = (position.0 + 1, position.1);
                }
                Direction::West => {
                    position = (position.0 - 1, position.1);
                }
            };

            history.insert(position);
        });

        history.len().to_string()
    }

    fn part_two(input: &str) -> String {
        let mut position = ((0, 0), (0, 0));
        let mut history = (HashSet::new(), HashSet::new());
        history.0.insert(position.0);
        history.1.insert(position.1);

        input.chars().enumerate().for_each(|(index, char)| {
            match (char.into(), index % 2) {
                (Direction::North, 0) => {
                    position.0 = (position.0 .0, position.0 .1 + 1);
                    history.0.insert(position.0);
                }
                (Direction::South, 0) => {
                    position.0 = (position.0 .0, position.0 .1 - 1);
                    history.0.insert(position.0);
                }
                (Direction::East, 0) => {
                    position.0 = (position.0 .0 + 1, position.0 .1);
                    history.0.insert(position.0);
                }
                (Direction::West, 0) => {
                    position.0 = (position.0 .0 - 1, position.0 .1);
                    history.0.insert(position.0);
                }
                (Direction::North, 1) => {
                    position.1 = (position.1 .0, position.1 .1 + 1);
                    history.1.insert(position.1);
                }
                (Direction::South, 1) => {
                    position.1 = (position.1 .0, position.1 .1 - 1);
                    history.1.insert(position.1);
                }
                (Direction::East, 1) => {
                    position.1 = (position.1 .0 + 1, position.1 .1);
                    history.1.insert(position.1);
                }
                (Direction::West, 1) => {
                    position.1 = (position.1 .0 - 1, position.1 .1);
                    history.1.insert(position.1);
                }
                _ => panic!("Unknown case to handle"),
            };
        });

        history.0.extend(history.1);

        history.0.len().to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_3.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day3Problem::part_one(input));
    println!("Part 2: {}", Day3Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2015::Problem;

    use crate::Day3Problem;

    #[test]
    fn part_1() {
        assert_eq!("2", Day3Problem::part_one(">"));
        assert_eq!("4", Day3Problem::part_one("^>v<"));
        assert_eq!("2", Day3Problem::part_one("^v^v^v^v^v"));
    }

    #[test]
    fn part_2() {
        assert_eq!("3", Day3Problem::part_two("^v"));
        assert_eq!("3", Day3Problem::part_two("^>v<"));
        assert_eq!("11", Day3Problem::part_two("^v^v^v^v^v"));
    }
}
