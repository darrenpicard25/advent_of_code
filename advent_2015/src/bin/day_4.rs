use advent_2015::Problem;

struct Day4Problem;

impl Problem for Day4Problem {
    fn part_one(input: &str) -> String {
        "TODO".to_string()
    }

    fn part_two(input: &str) -> String {
        "TODO".to_string()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./advent_2015/data/day_4.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day4Problem::part_one(input));
    println!("Part 2: {}", Day4Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use advent_2015::Problem;

    use crate::Day4Problem;

    #[test]
    fn part_1() {
        assert_eq!("2", Day4Problem::part_one(">"));
        assert_eq!("4", Day4Problem::part_one("^>v<"));
        assert_eq!("2", Day4Problem::part_one("^v^v^v^v^v"));
    }

    #[test]
    fn part_2() {
        assert_eq!("3", Day4Problem::part_two("^v"));
        assert_eq!("3", Day4Problem::part_two("^>v<"));
        assert_eq!("11", Day4Problem::part_two("^v^v^v^v^v"));
    }
}
