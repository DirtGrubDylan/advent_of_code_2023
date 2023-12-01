pub mod util;

mod day_1;

use std::io::{self, Write};

fn print_seperator() {
    println!("-------------------------------");
}

fn run_day(day: u32) {
    match day {
        1 => day_1::run(),
        _ => unimplemented!("I haven't done that day yet :("),
    }
}

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
