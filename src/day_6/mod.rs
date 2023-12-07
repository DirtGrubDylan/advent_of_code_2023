mod race;

use crate::util::file_reader::to_string_vector;

use race::Race;

pub fn run() {
    let input = to_string_vector("inputs/day_6.txt").expect("Something went wrong with Day 6!");

    println!("Day 6 Part 1: {}", part_1(&input));
    println!("Day 6 Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> f64 {
    get_races(input)
        .iter()
        .map(Race::number_of_pressing_milliseconds_to_beat_record)
        .product()
}

fn part_2(input: &[String]) -> f64 {
    get_race_with_kerning(input).number_of_pressing_milliseconds_to_beat_record()
}

fn get_races(input: &[String]) -> Vec<Race> {
    let times = input[0]
        .split_whitespace()
        .skip(1)
        .map(|item| item.parse::<f64>().unwrap());
    let records = input[1]
        .split_whitespace()
        .skip(1)
        .map(|item| item.parse::<f64>().unwrap());

    times.zip(records).map(Race::from).collect()
}

fn get_race_with_kerning(input: &[String]) -> Race {
    let time = get_u32_with_kerning(&input[0]);
    let record = get_u32_with_kerning(&input[1]);

    Race::from((time, record))
}

fn get_u32_with_kerning(input: &str) -> f64 {
    let mut result = 0.0;

    for digit in input.chars().filter_map(|c| c.to_digit(10)) {
        result = result * 10.0 + f64::from(digit);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_6.txt")
            .expect("Something went wrong with Day 6 Part 1 Test!");

        let expected = 288.0;

        let result = part_1(&input);

        assert!((result - expected).abs() < f64::EPSILON);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_6.txt")
            .expect("Something went wrong with Day 6 Part 2 Test!");

        let expected = 71_503.0;

        let result = part_2(&input);

        assert!((result - expected).abs() < f64::EPSILON);
    }
}
