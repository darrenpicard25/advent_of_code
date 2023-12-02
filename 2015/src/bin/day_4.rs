use advent_2015::Problem;
use hex::ToHex;
use md5hash::MD5Hasher;

struct Day4Problem;

impl Problem for Day4Problem {
    fn part_one(input: &str) -> String {
        let mut answer = 0;
        loop {
            let mut hasher = MD5Hasher::new();
            let value = format!("{input}{answer}");
            hasher.digest(&value);

            let results = hasher.finish();

            if results.encode_hex::<String>().starts_with("00000") {
                break;
            }
            answer += 1;
        }

        answer.to_string()
    }

    fn part_two(input: &str) -> String {
        let mut answer = 0;
        loop {
            let mut hasher = MD5Hasher::new();
            let value = format!("{input}{answer}");
            hasher.digest(&value);

            let results = hasher.finish();

            if results.encode_hex::<String>().starts_with("000000") {
                break;
            }
            answer += 1;
        }

        answer.to_string()
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
        assert_eq!("609043", Day4Problem::part_one("abcdef"));
        assert_eq!("1048970", Day4Problem::part_one("pqrstuv"));
    }

    #[test]
    fn part_2() {
        assert_eq!("3", Day4Problem::part_two("^v"));
        assert_eq!("3", Day4Problem::part_two("^>v<"));
        assert_eq!("11", Day4Problem::part_two("^v^v^v^v^v"));
    }
}
