use std::fs::{ self };
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("[--Input File--]")?;
    let number = get_numbers(&contents);

    println!("And the result is {number}");

    Ok(())
}
