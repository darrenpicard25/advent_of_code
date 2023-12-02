use aoc_2015::Problem;

#[derive(Debug)]
struct Point(usize, usize);

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut s = value.split(',');

        Self(
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().parse().unwrap(),
        )
    }
}

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        match value {
            "turn off" => Action::TurnOff,
            "turn on" => Action::TurnOn,
            "toggle" => Action::Toggle,
            _ => panic!("Not implemented"),
        }
    }
}

impl From<String> for Action {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "turn off" => Action::TurnOff,
            "turn on" => Action::TurnOn,
            "toggle" => Action::Toggle,
            _ => panic!("Not implemented"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    initial_point: Point,
    final_point: Point,
    action: Action,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut iter = value.split_ascii_whitespace().peekable();

        let action: Action = if iter.peek().unwrap() == &"toggle" {
            iter.next().unwrap().into()
        } else {
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();

            format!("{first} {second}").into()
        };

        let initial_point = iter.next().unwrap().into();
        iter.next();
        let final_point = iter.next().unwrap().into();

        Self {
            action,
            initial_point,
            final_point,
        }
    }
}

struct Day6Problem;

impl Problem for Day6Problem {
    fn part_one(input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| line.into())
            .collect::<Vec<Instruction>>();

        let mut lights = vec![false; 1000 * 1000];

        // println!("{lights:?}");

        for instruction in instructions {
            for x in instruction.initial_point.0..=instruction.final_point.0 {
                for y in instruction.initial_point.1..=instruction.final_point.1 {
                    let index = (x * 1000) + y;

                    match instruction.action {
                        Action::TurnOn => lights[index] = true,
                        Action::TurnOff => lights[index] = false,
                        Action::Toggle => {
                            let current = lights[index];
                            lights[index] = !current
                        }
                    }
                }
            }
        }

        lights.iter().filter(|is_on| **is_on).count().to_string()
    }

    fn part_two(input: &str) -> String {
        let instructions = input
            .lines()
            .map(|line| line.into())
            .collect::<Vec<Instruction>>();

        let mut lights = vec![0u8; 1000 * 1000];

        for instruction in instructions {
            for x in instruction.initial_point.0..=instruction.final_point.0 {
                for y in instruction.initial_point.1..=instruction.final_point.1 {
                    let index = (x * 1000) + y;

                    match instruction.action {
                        Action::TurnOn => lights[index] += 1,
                        Action::TurnOff => {
                            let current = lights[index];

                            lights[index] = current.checked_sub(1).unwrap_or(0);
                        }
                        Action::Toggle => lights[index] += 2,
                    }
                }
            }
        }

        lights
            .iter()
            .fold(0u64, |mut acc, value| {
                acc += *value as u64;
                acc
            })
            .to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2015/data/day_6.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day6Problem::part_one(input));
    println!("Part 2: {}", Day6Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2015::Problem;

    use crate::Day6Problem;

    #[test]
    fn part_1() {
        let input = "turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500";
        assert_eq!("998996", Day6Problem::part_one(input));
    }

    #[test]
    fn part_2() {
        let input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999";

        assert_eq!("2000001", Day6Problem::part_two(input));
    }
}
