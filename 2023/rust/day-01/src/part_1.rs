use crate::error::AocResult;

pub fn process(input: &str) -> AocResult<String> {
    Ok(input
        .lines()
        .map(|line| {
            let mut num_iterator = line.chars().filter(|char| char.is_digit(10));

            let first_digit = num_iterator.next().expect("Error getting first digit");
            match num_iterator.last() {
                Some(last_digit) => format!("{first_digit}{last_digit}"),
                None => format!("{first_digit}{first_digit}"),
            }
            .parse::<u32>()
            .expect("Failed to parse number")
        })
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("142", process(file)?);
        Ok(())
    }
}
