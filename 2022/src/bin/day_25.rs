use aoc_2022::Problem;

struct Base10(i64);

#[derive(Debug)]
enum Base5 {
    Two,
    One,
    Zero,
    MinusOne,
    MinusTwo,
}

impl TryFrom<char> for Base5 {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Base5::Two),
            '1' => Ok(Base5::One),
            '0' => Ok(Base5::Zero),
            '-' => Ok(Base5::MinusOne),
            '=' => Ok(Base5::MinusTwo),
            c => Err(format!("Unknown Character: {c}")),
        }
    }
}

impl From<Base5> for i64 {
    fn from(value: Base5) -> Self {
        match value {
            Base5::Two => 2,
            Base5::One => 1,
            Base5::Zero => 0,
            Base5::MinusOne => -1,
            Base5::MinusTwo => -2,
        }
    }
}

impl From<Vec<Base5>> for Base10 {
    fn from(value: Vec<Base5>) -> Self {
        value
            .into_iter()
            .rev()
            .enumerate()
            .fold(Base10(0), |mut value, (index, current)| {
                let position = 5_i64.pow(index as u32);
                let position_value = position * i64::from(current);
                value.0 += position_value;

                value
            })
    }
}

struct Day25Problem;

impl Day25Problem {}

impl Problem for Day25Problem {
    fn part_one(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let line = line
                    .trim()
                    .chars()
                    .filter_map(|char| Base5::try_from(char).ok())
                    .collect::<Vec<Base5>>();

                let value = Base10::from(line);

                value.0
            })
            .sum::<i64>()
            .to_string()
    }

    fn part_two(input: &str) -> String {
        "TODO".into()
    }
}

fn main() {
    println!("{:?}", std::env::current_dir());
    let file = std::fs::read("./aoc_2022/data/day_25.txt").unwrap();
    let input = std::str::from_utf8(&file).unwrap();

    println!("Part 1: {}", Day25Problem::part_one(input));
    println!("Part 2: {}", Day25Problem::part_two(input));
}

#[cfg(test)]
mod tests {
    use aoc_2022::Problem;

    use crate::Day25Problem;

    #[test]
    fn part_1() {
        assert_eq!(Day25Problem::part_one("2=-01"), "976");
        assert_eq!(Day25Problem::part_one("1=-0-2"), "1747");
        assert_eq!(Day25Problem::part_one("12111"), "906");
        assert_eq!(Day25Problem::part_one("2=0="), "198");
        assert_eq!(Day25Problem::part_one("21"), "11");
        assert_eq!(Day25Problem::part_one("2=01"), "201");
        assert_eq!(Day25Problem::part_one("111"), "31");
        assert_eq!(Day25Problem::part_one("20012"), "1257");
        assert_eq!(Day25Problem::part_one("112"), "32");
        assert_eq!(Day25Problem::part_one("1=-1="), "353");
        assert_eq!(Day25Problem::part_one("1-12"), "107");
        assert_eq!(Day25Problem::part_one("12"), "7");
        assert_eq!(Day25Problem::part_one("1="), "3");
        assert_eq!(Day25Problem::part_one("122"), "37");

        let input = r#"
                                1=-0-2
                                12111
                                2=0=
                                21
                                2=01
                                111
                                20012
                                112
                                1=-1=
                                1-12
                                12
                                1=
                                122"#;

        assert_eq!(Day25Problem::part_one(input), "4890");
    }

    #[test]
    fn part_2() {
        let input = "";

        assert_eq!("2000001", Day25Problem::part_two(input));
    }
}
