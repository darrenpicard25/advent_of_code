use aoc_2015::Problem;

struct Day1Problem;

impl Problem for Day1Problem {
    fn part_one(input: &str) -> String {
        input
            .chars()
            .map(|char| match char {
                ')' => -1,
                '(' => 1,
                _ => 0,
            })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        let mut floor = 0;
        let iterator = input.chars().map(|char| match char {
            ')' => -1,
            '(' => 1,
            _ => 0,
        });
        for (index, direction) in iterator.enumerate() {
            floor += direction;

            if floor < 0 {
                return (1 + index).to_string();
            }
        }

        floor.to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2015/data/day_1.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day1Problem::part_one(input));
    println!("Part 2: {}", Day1Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2015::Problem;

    use crate::Day1Problem;

    #[test]
    fn part_1() {
        assert_eq!("0", Day1Problem::part_one("(())"));
        assert_eq!("0", Day1Problem::part_one("()()"));
        assert_eq!("3", Day1Problem::part_one("))((((("));
        assert_eq!("-3", Day1Problem::part_one(")())())"));
    }

    #[test]
    fn part_2() {
        assert_eq!("1", Day1Problem::part_two(")"));
        assert_eq!("5", Day1Problem::part_two("()())"));
    }
}
