mod calibration_value;

use crate::util::file_reader::to_string_vector;

use calibration_value::CalibrationValue;

pub fn run() {
    let input = to_string_vector("inputs/day_1.txt").expect("Something went wrong with Day 1!");

    println!("Day 1 Part 1: {:?}", part_1(&input));
    println!("Day 1 Part 2: {:?}", part_2(&input));
}

fn part_1(inputs: &[String]) -> CalibrationValue {
    inputs
        .iter()
        .map(|input| input.parse::<CalibrationValue>().unwrap())
        .sum()
}

fn part_2(input: &[String]) -> CalibrationValue {
    let curated_input = curate_data(input);

    curated_input
        .iter()
        .map(|input| input.parse::<CalibrationValue>().unwrap())
        .sum()
}

fn curate_data(input: &[String]) -> Vec<String> {
    input.iter().map(|line| curate_line(line)).collect()
}

fn curate_line(line: &str) -> String {
    line.replace("oneight", "18")
        .replace("twone", "21")
        .replace("threeight", "38")
        .replace("fiveight", "58")
        .replace("sevenine", "79")
        .replace("eightwo", "82")
        .replace("eighthree", "83")
        .replace("nineight", "98")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let inputs = to_string_vector("test_inputs/day_1_part_1.txt")
            .expect("Something went wrong with Day 1 Part 1 Test!");

        let expected = CalibrationValue(142);

        let result = part_1(&inputs);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let inputs = to_string_vector("test_inputs/day_1_part_2.txt")
            .expect("Something went wrong with Day 1 Part 2 Test!");

        let expected = CalibrationValue(281);

        let result = part_2(&inputs);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_currate_line() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "jbnrs5bgmsixeightxzjznzkhpvsix5twoneb",
        ];

        let expected = vec![
            "219".to_string(),
            "823".to_string(),
            "abc123xyz".to_string(),
            "x2134".to_string(),
            "49872".to_string(),
            "z18234".to_string(),
            "7pqrst6teen".to_string(),
            "jbnrs5bgm68xzjznzkhpv6521b".to_string(),
        ];

        let result: Vec<String> = input.iter().map(|line| curate_line(line)).collect();

        for (result_line, expected_line) in result.iter().zip(expected.iter()) {
            assert_eq!(result_line, expected_line);
        }
    }
}
