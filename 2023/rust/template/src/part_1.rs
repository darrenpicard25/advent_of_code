use crate::error::AocResult;

pub fn process(_input: &str) -> AocResult<String> {
    todo!("{{project-name}} - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> AocResult<()> {
        let file = include_str!("../data/part_1_test_input.txt");
        assert_eq!("", process(file)?);
        Ok(())
    }
}
