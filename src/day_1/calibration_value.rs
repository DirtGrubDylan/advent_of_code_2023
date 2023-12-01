use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CalibrationValue(pub u32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ParseError;

impl Add for CalibrationValue {
    type Output = CalibrationValue;

    fn add(self, rhs: Self) -> Self::Output {
        CalibrationValue(self.0 + rhs.0)
    }
}

impl Sum for CalibrationValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(CalibrationValue(0), |a, b| a + b)
    }
}

impl FromStr for CalibrationValue {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        for c_digit in input.chars().map(|c| c.to_digit(10)) {
            first_digit = first_digit.or(c_digit);
            last_digit = c_digit.or(last_digit);
        }

        match (first_digit, last_digit) {
            (Some(x), Some(y)) => Ok(CalibrationValue(x * 10 + y)),
            _ => Err(ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_ok() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let expected = vec![
            CalibrationValue(12),
            CalibrationValue(38),
            CalibrationValue(15),
            CalibrationValue(77),
        ];

        let result: Vec<CalibrationValue> =
            inputs.iter().map(|input| input.parse().unwrap()).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_from_str_err() {
        let no_numbers = "trebuchet";

        assert!(no_numbers.parse::<CalibrationValue>().is_err());
    }

    #[test]
    fn test_from_str_sum() {
        let values = vec![
            CalibrationValue(12),
            CalibrationValue(38),
            CalibrationValue(15),
            CalibrationValue(77),
        ];

        let expected = CalibrationValue(142);

        let result: CalibrationValue = values.into_iter().sum();

        assert_eq!(result, expected);
    }
}
