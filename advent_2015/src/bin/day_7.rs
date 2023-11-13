use std::collections::HashMap;

use advent_2015::Problem;

#[derive(Debug)]
enum Value<'a> {
    Number(u16),
    Reference(&'a str),
    // Command(Box<Command<'a>>),
}

impl<'a> Value<'a> {
    fn calculate(&self, variables: &HashMap<&str, Command>) -> u16 {
        match self {
            Value::Number(value) => *value,
            Value::Reference(key) => variables
                .get(key)
                .expect("Unable to find {key}")
                .calculate(variables),
            // Value::Command(command) => command.calculate(variables),
        }
    }
}

impl<'a> From<&&'a str> for Value<'a> {
    fn from(value: &&'a str) -> Self {
        value
            .parse::<u16>()
            .map(|number| Value::Number(number))
            .unwrap_or_else(|_| Value::Reference(value))
    }
}

#[derive(Debug)]
struct Expression<'a> {
    command: Command<'a>,
    destination: &'a str,
}

#[derive(Debug)]
enum Command<'a> {
    AND(Value<'a>, Value<'a>),
    OR(Value<'a>, Value<'a>),
    LSHIFT(Value<'a>, Value<'a>),
    RSHIFT(Value<'a>, Value<'a>),
    NOT(Value<'a>),
    ASSIGN(Value<'a>),
}

impl<'a> Command<'a> {
    fn calculate(&self, variables: &HashMap<&str, Command>) -> u16 {
        dbg!(self);
        match self {
            Command::AND(arg1, arg2) => arg1.calculate(variables) & arg2.calculate(variables),
            Command::OR(arg1, arg2) => arg1.calculate(variables) | arg2.calculate(variables),
            Command::LSHIFT(arg1, arg2) => arg1.calculate(variables) << arg2.calculate(variables),
            Command::RSHIFT(arg1, arg2) => arg1.calculate(variables) >> arg2.calculate(variables),
            Command::NOT(arg) => !arg.calculate(variables),
            Command::ASSIGN(arg) => arg.calculate(variables),
        }
    }
}

struct Day7Problem;

impl<'a> Day7Problem {
    fn calculate(key: &'static str, variables: &HashMap<&str, Command>) -> u16 {
        variables
            .get(key)
            .expect("Unable to find {key}")
            .calculate(variables)
    }

    fn parse(input: &'a str) -> Vec<Expression> {
        input
            .lines()
            .filter_map(|line| {
                match line
                    .split_ascii_whitespace()
                    .collect::<Vec<&str>>()
                    .as_slice()
                {
                    ["NOT", arg, "->", destination] => Some(Expression {
                        command: Command::NOT(arg.into()),
                        destination,
                    }),
                    [arg, "->", destination] => Some(Expression {
                        command: Command::ASSIGN(arg.into()),
                        destination,
                    }),
                    [arg_1, "RSHIFT", arg_2, "->", destination] => Some(Expression {
                        command: Command::RSHIFT(arg_1.into(), arg_2.into()),
                        destination,
                    }),
                    [arg_1, "LSHIFT", arg_2, "->", destination] => Some(Expression {
                        command: Command::LSHIFT(arg_1.into(), arg_2.into()),
                        destination,
                    }),
                    [arg_1, "AND", arg_2, "->", destination] => Some(Expression {
                        command: Command::AND(arg_1.into(), arg_2.into()),
                        destination,
                    }),
                    [arg_1, "OR", arg_2, "->", destination] => Some(Expression {
                        command: Command::OR(arg_1.into(), arg_2.into()),
                        destination,
                    }),
                    _ => None,
                }
            })
            .collect()
    }
}

impl Problem for Day7Problem {
    fn part_one(input: &str) -> String {
        let mut variables: HashMap<&str, Command> = HashMap::new();
        let instructions = Day7Problem::parse(input);

        for instruction in instructions {
            variables.insert(instruction.destination, instruction.command);
        }

        Day7Problem::calculate("a", &variables).to_string()
    }

    fn part_two(input: &str) -> String {
        "TODO".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_7.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day7Problem::part_one(input));
    println!("Part 2: {}", Day7Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2015::Problem;

    use crate::Day7Problem;

    #[test]
    fn part_1() {
        let input = r#"
        lx -> w
        123 -> x
        456 -> y
        x AND y -> d
        x OR y -> e
        x LSHIFT 2 -> f
        y RSHIFT 2 -> g
        NOT x -> h
        NOT y -> i
        45 -> lx
        "#;

        assert_eq!("45", Day7Problem::part_one(input));
    }

    #[test]
    fn part_2() {
        let input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999";

        assert_eq!("2000001", Day7Problem::part_two(input));
    }
}
