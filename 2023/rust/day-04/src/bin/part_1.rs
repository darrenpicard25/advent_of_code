use day_04::{error::AocResult, part_1::process};

fn main() -> AocResult<()> {
    let file = include_str!("../../data/input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
