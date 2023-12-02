use crate::error::AocResult;

pub fn process(input: &str) -> AocResult<String> {
    Ok(input
        .lines()
        .map(|line| {
            let mut index = 0;
            let line_iterator = std::iter::from_fn(move || {
                let result = match &line[index..] {
                    slice if slice.starts_with("one") => Some('1'),
                    slice if slice.starts_with("two") => Some('2'),
                    slice if slice.starts_with("three") => Some('3'),
                    slice if slice.starts_with("four") => Some('4'),
                    slice if slice.starts_with("five") => Some('5'),
                    slice if slice.starts_with("six") => Some('6'),
                    slice if slice.starts_with("seven") => Some('7'),
                    slice if slice.starts_with("eight") => Some('8'),
                    slice if slice.starts_with("nine") => Some('9'),
                    slice => slice.chars().next(),
                };

                index += 1;
                result
            });

            let mut num_iterator = line_iterator.filter(|char| char.is_digit(10));

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
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("281", process(file)?);
        Ok(())
    }
}
