use day_03::{error::AocResult, part_1::process};

fn main() -> AocResult<()> {
    let file = include_str!("../../data/part_1_input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}