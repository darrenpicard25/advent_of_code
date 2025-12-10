use day_03::{error::AocResult, part_2::process};

fn main() -> AocResult<()> {
    let file = include_str!("../../data/input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}