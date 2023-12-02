use {{crate_name}}::{error::AocResult, part_2::process};

fn main() -> AocResult<()> {
    let file = include_str!("../../data/part_2_input.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}