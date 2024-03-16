use std::fs::{ self };
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("[--Input File--]")?;
    let number = get_numbers(&contents);

    println!("And the result is {number}");

    Ok(())
}

// Match the first digit and the last digit in the line to create a number
//If there is one number then it is both the first digit and the last digit

fn get_numbers(contents: &String) -> i32 {
    let mut accum: i32 = 0;

    for line in contents.lines() {
        let line_number: i32 = get_line_digits(&line);
        accum = accum + line_number;
    }

    accum
}

fn get_line_digits(line: &str) -> i32 {
    let mut first: String = String::new();
    let mut last: String = String::new();

    for character in line.chars() {
        if character.is_ascii_digit() {
            first = character.to_string();
            break;
        }
    }
    for character in line.chars().rev() {
        if character.is_ascii_digit() {
            last = character.to_string();
            break;
        }
    }
    let join = first + &last;
    join.parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn return_number_basic() {
        let number_char = String::from("1fdlkgjh3");
        println!("{}", get_line_digits(&number_char));
        assert_eq!(13, get_line_digits(&number_char));
    }
    #[test]
    fn return_number_single() {
        let number_char = String::from("jejn1djfvn");
        assert_eq!(11, get_line_digits(&number_char));
    }

    #[test]
    fn get_from_lines() {
        let lines = String::from(
            "fivepqxlpninevh2xxsnsgg63pbvdnqptmg\nnvcchgjnine9sixtwompfrp\n685"
        );
        assert_eq!(23 + 99 + 65, get_numbers(&lines))
    }
}
