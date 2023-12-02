use std::collections::HashMap;

use advent_2015::Problem;

struct Day7Problem;

impl<'a> Day7Problem {}

impl Problem for Day7Problem {
    fn part_one(input: &str) -> String {
        let x = input.lines().map(|line| {
            let num_characters_code = line.trim().chars().count();
            let mem_iterator = line.trim().chars().fold(0, |acc, char| {});
        });
        todo!()
    }

    fn part_two(input: &str) -> String {
        "TODO".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_8.txt").unwrap();
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
        let input = r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#;

        assert_eq!("12", Day7Problem::part_one(input));
    }

    #[test]
    fn part_2() {
        let input = "turn on 0,0 through 0,0\ntoggle 0,0 through 999,999";

        assert_eq!("2000001", Day7Problem::part_two(input));
    }
}
