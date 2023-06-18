use std::collections::HashSet;

use advent_2015::Problem;

struct Day5Problem;

impl Problem for Day5Problem {
    fn part_one(input: &str) -> String {
        input
            .lines()
            .filter(|name| {
                let contains_3_vowels =
                    name.chars().filter(|char| "aeiou".contains(*char)).count() >= 3;

                let contains_repeat_char = name
                    .as_bytes()
                    .windows(2)
                    .any(|window| window[0] == window[1]);

                let contain_bad_string = name.as_bytes().windows(2).any(|window| {
                    [
                        "ab".as_bytes(),
                        "cd".as_bytes(),
                        "pq".as_bytes(),
                        "xy".as_bytes(),
                    ]
                    .contains(&window)
                });

                contains_3_vowels && contains_repeat_char && !contain_bad_string
            })
            .count()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        input
            .lines()
            .filter(|name| {
                let mut tracker = HashSet::new();
                let line = name.as_bytes();

                let contains_matching_pair = line.windows(2).enumerate().any(|(index, window)| {
                    if window.first() == window.last()
                        && window.first() == line.get(index + 2)
                        && window.first() != line.get(index + 3)
                    {
                        return false;
                    }

                    !tracker.insert(window.get(0..2))
                });

                let contain_repeat_char_separated = line
                    .windows(3)
                    .any(|window| window.first() == window.last());

                contains_matching_pair && contain_repeat_char_separated
            })
            .count()
            .to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_5.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day5Problem::part_one(input));
    println!("Part 2: {}", Day5Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2015::Problem;

    use crate::Day5Problem;

    #[test]
    fn part_1() {
        assert_eq!("1", Day5Problem::part_one("ugknbfddgicrmopn"));
        assert_eq!("1", Day5Problem::part_one("aaa"));
        assert_eq!("0", Day5Problem::part_one("jchzalrnumimnmhp"));
        assert_eq!("0", Day5Problem::part_one("haegwjzuvuyypxyu"));
        assert_eq!("0", Day5Problem::part_one("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_2() {
        assert_eq!("1", Day5Problem::part_two("qjhvhtzxzqqjkmpb"));
        assert_eq!("1", Day5Problem::part_two("xxyxx"));
        assert_eq!("0", Day5Problem::part_two("uurcxstgmygtbstg"));
        assert_eq!("0", Day5Problem::part_two("ieodomkazucvgmuy"));
        assert_eq!("0", Day5Problem::part_two("aaa"));
    }
}
