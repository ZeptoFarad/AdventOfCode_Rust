extern crate aoc12;

use std::process;

fn main() {
    if let Err(e) = aoc1::run() {
        eprint!("Application Error: {e}");
        process::exit(1);
    }
}
