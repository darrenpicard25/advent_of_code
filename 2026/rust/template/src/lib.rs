use common::AocResult;

pub const INPUT: &str = include_str!("../data/input.txt");

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &str) -> AocResult<usize> {
    let _parsed = parse(input);
    todo!("{{project-name}} - part 1")
}

pub fn part2(input: &str) -> AocResult<usize> {
    let _parsed = parse(input);
    todo!("{{project-name}} - part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
";

    #[test]
    fn test_part1() -> AocResult<()> {
        assert_eq!(part1(EXAMPLE)?, 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> AocResult<()> {
        assert_eq!(part2(EXAMPLE)?, 0);
        Ok(())
    }
}
