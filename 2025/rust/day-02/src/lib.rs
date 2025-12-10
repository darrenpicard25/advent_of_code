use std::ops::RangeInclusive;

pub mod error;

pub mod part_1;
pub mod part_2;

pub fn extract_ranges(input: &str) -> error::AocResult<Vec<RangeInclusive<usize>>> {
    input
        .split(',')
        .map(|item| {
            let Some((first_id, second_id)) = item.split_once('-') else {
                eprintln!("Unable to split item on delemeter - : {item}");
                return Err(error::AocError::IoError);
            };

            let first_id = match first_id.parse::<usize>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Unknown parsing error on first id {first_id}:{e}");

                    return Err(error::AocError::IoError);
                }
            };

            let second_id = match second_id.parse::<usize>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Unknown parsing error on second id {second_id}:{e}");

                    return Err(error::AocError::IoError);
                }
            };

            Ok(first_id..=second_id)
        })
        .collect::<error::AocResult<Vec<RangeInclusive<usize>>>>()
}
