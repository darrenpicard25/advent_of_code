use crate::{error::AocResult, DialPosition, Instruction};

pub fn process(input: &str) -> AocResult<String> {
    let mut dial = DialPosition::new();

    let instructions = input
        .lines()
        .map(Instruction::try_from)
        .collect::<AocResult<Vec<Instruction>>>()?;

    let mut counter = 0;

    for instruction in instructions {
        let rotaions = match instruction {
            Instruction::Left(amount) => dial.move_left(amount),
            Instruction::Right(amount) => dial.move_right(amount),
        };

        counter += rotaions;
    }

    Ok(counter.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_2_test_input.txt");
        assert_eq!("6", process(file)?);
        Ok(())
    }

    #[test]
    fn test_looping() -> AocResult<()> {
        assert_eq!("10", process(r#"R1000"#)?);
        Ok(())
    }

    #[test]
    fn custom_test() -> AocResult<()> {
        assert_eq!("1", process("R50\nL50")?);
        Ok(())
    }
}
