use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Oasis {
    reports: Vec<Report>,
}

impl Oasis {
    pub fn new(input: &[String]) -> Self {
        Oasis {
            reports: input.iter().filter_map(|row| row.parse().ok()).collect(),
        }
    }

    pub fn sum_of_next_values(&self) -> i32 {
        self.reports.iter().map(Report::next_value).sum()
    }

    pub fn sum_of_previous_values(&self) -> i32 {
        self.reports.iter().map(Report::previous_value).sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct Report {
    value_history: Vec<i32>,
}

impl Report {
    fn new(input: &[i32]) -> Self {
        Report {
            value_history: input.to_vec(),
        }
    }

    fn next_value(&self) -> i32 {
        self.extrapolated_values()
            .into_iter()
            .filter_map(|values| values.last().copied())
            .sum()
    }

    fn previous_value(&self) -> i32 {
        self.extrapolated_values()
            .into_iter()
            .filter_map(|values| values.first().copied())
            .enumerate()
            .fold(
                0,
                |acc, (idx, x)| {
                    if idx % 2 == 0 {
                        acc + x
                    } else {
                        acc - x
                    }
                },
            )
    }

    fn extrapolated_values(&self) -> Vec<Vec<i32>> {
        let mut extrapolated_values = vec![self.value_history.clone()];

        loop {
            let last_extrapolation = extrapolated_values.last().unwrap();

            if last_extrapolation.iter().all(|&value| value == 0) {
                break;
            }

            extrapolated_values.push(
                last_extrapolation
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect(),
            );
        }

        extrapolated_values
    }
}

impl FromStr for Report {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let values: Vec<i32> = input.split(' ').filter_map(|s| s.parse().ok()).collect();

        Ok(Report::new(&values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_from_str() {
        let input = "-4 -6 -2 10 22 13 -44 -158 -278 -243 250 1613";

        let expected = Report::new(&[-4, -6, -2, 10, 22, 13, -44, -158, -278, -243, 250, 1_613]);

        let result: Report = input.parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_report_next_value() {
        let report = Report::new(&[-4, -6, -2, 10, 22, 13, -44, -158, -278, -243, 250, 1_613]);

        let expected = 4_196;

        let result = report.next_value();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_report_previous_value() {
        let report = Report::new(&[10, 13, 16, 21, 30, 45]);

        let expected = 5;

        let result = report.previous_value();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_oasis_sum_of_next_values() {
        let input = [
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ];

        let oasis = Oasis::new(&input);

        let expected = 114;

        let result = oasis.sum_of_next_values();

        assert_eq!(result, expected);
    }
}
