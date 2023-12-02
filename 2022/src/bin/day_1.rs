use advent_2022::Problem;

struct Day1Problem;

impl Problem for Day1Problem {
    fn part_one(input: &str) -> String {
        "TODO".into()
    }

    fn part_two(input: &str) -> String {
        "TODO".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_7.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day1Problem::part_one(input));
    println!("Part 2: {}", Day1Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2022::Problem;

    use crate::Day1Problem;

    #[test]
    fn part_1() {
        let input = r#""#;

        assert_eq!("45", Day1Problem::part_one(input));
    }

    #[test]
    fn part_2() {
        let input = "";

        assert_eq!("2000001", Day1Problem::part_two(input));
    }
}
