use crate::{error::AocResult, DialPosition, Instruction};

pub fn process(input: &str) -> AocResult<String> {
    let mut dial = DialPosition::new();

    let instructions = input
        .lines()
        .map(Instruction::try_from)
        .collect::<AocResult<Vec<Instruction>>>()?;

    let mut counter = 0;

    for instruction in instructions {
        // dbg!(&instruction);

        match instruction {
            Instruction::Left(amount) => dial.move_left(amount),
            Instruction::Right(amount) => dial.move_right(amount),
        };

        if dial.is_pointing_at_zero() {
            counter += 1;
        }
    }
    Ok(counter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("3", process(file)?);
        Ok(())
    }
}
