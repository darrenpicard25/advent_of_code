use std::collections::{HashMap, HashSet};

use advent_2020::Problem;

struct Day6Problem;

impl Problem for Day6Problem {
    fn part_one(input: &str) -> String {
        input
            .split("\n\n")
            .into_iter()
            .map(|group| {
                let mut count = HashSet::new();
                group.lines().for_each(|person| {
                    person.trim().chars().for_each(|question| {
                        count.insert(question);
                    })
                });

                count.len()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        input
            .split("\n\n")
            .into_iter()
            .map(|group| {
                let mut counter = HashMap::new();
                let group_size = group.lines().count();
                group.lines().for_each(|person| {
                    person.trim().chars().for_each(|question| {
                        counter
                            .entry(question)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    })
                });

                counter
                    .into_iter()
                    .filter(|(_, count)| *count == group_size)
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2020/data/day_6.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day6Problem::part_one(input));
    println!("Part 2: {}", Day6Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2020::Problem;

    use crate::Day6Problem;

    const INPUT: &'static str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
    #[test]
    fn part_1() {
        assert_eq!("11", Day6Problem::part_one(INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!("6", Day6Problem::part_two(INPUT));
    }
}
