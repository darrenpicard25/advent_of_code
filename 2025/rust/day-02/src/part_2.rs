use crate::{error::AocResult, extract_ranges};

pub fn process(input: &str) -> AocResult<String> {
    let ranges = extract_ranges(input)?;

    let mut count = 0;

    for range in ranges {
        println!("Range: {range:?}");
        for num in range {
            if num / 10 == 0 {
                continue;
            }
            let string_num = num.to_string();

            'sizing_loop: for size in 1..=string_num.len() / 2 {
                if num == 1010 {
                    println!("{:?}", string_num.as_bytes());
                }
                let mut iterator = string_num.as_bytes().chunks(size);

                if let Some(first) = iterator.next() {
                    if num == 1010 {
                        println!("first: {first:?}");
                        println!("{iterator:?}")
                    }
                    if iterator.all(|item| item == first) {
                        println!("Number: {num}");
                        count += num;
                        break 'sizing_loop;
                    }
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
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("4174379265", process(file)?);
        Ok(())
    }
}
