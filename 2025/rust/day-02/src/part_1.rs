use crate::{error::AocResult, extract_ranges};

pub fn process(input: &str) -> AocResult<String> {
    let ranges = extract_ranges(input)?;

    let mut count = 0;

    for range in ranges {
        for num in range {
            let string_num = num.to_string();

            if string_num.len() % 2 == 0 {
                let (front, back) = string_num.split_at(string_num.len() / 2);
                if front == back {
                    count += num;
                }
            }
        }
    }

    Ok(count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("1227775554", process(file)?);
        Ok(())
    }
}
