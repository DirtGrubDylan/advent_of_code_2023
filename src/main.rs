pub mod util;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use std::io::{self, Write};

fn print_seperator() {
    println!("-------------------------------------");
}

fn run_day(day: u32) {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        4 => day_4::run(),
        5 => day_5::run(),
        6 => day_6::run(),
        _ => unimplemented!("I haven't done that day yet :("),
    }
}

/// Gets the user input.
///
/// # Panics
///
/// If line couldn't be flushed and/or stdin couldn't be read/parsed.
#[must_use]
pub fn get_user_input() -> u32 {
    let mut input_buffer = String::new();

    io::stdout().flush().expect("Could not flush stdout!");

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read user input!");

    input_buffer
        .trim()
        .parse::<u32>()
        .expect("Failed to parse user_input!")
}

fn main() {
    print_seperator();

    print!("Please choose a day to run (1-25): ");

    let input = get_user_input();

    print_seperator();

    run_day(input);

    print_seperator();
}
