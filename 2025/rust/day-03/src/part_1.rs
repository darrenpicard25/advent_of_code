use crate::error::AocResult;

pub fn process(input: &str) -> AocResult<String> {
    let answer: usize = input
        .lines()
        .map(|line| {
            let mut max_value = 0;

            for (starting_index, char) in line.chars().enumerate() {
                if let Some(remaining_line) = line.get((starting_index + 1)..) {
                    for char_2 in remaining_line.chars() {
                        let num = format!("{char}{char_2}").parse::<usize>().unwrap();

                        if num > max_value {
                            max_value = num;
                        }
                    }
                }
            }

            max_value
        })
        .sum();

    Ok(answer.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("357", process(file)?);
        Ok(())
    }
}
