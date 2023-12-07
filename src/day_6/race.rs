use std::convert::From;

#[derive(Debug, PartialEq)]
pub struct Race {
    time_limit_ms: f64,
    distance_record_mm: f64,
}

impl Race {
    pub fn number_of_pressing_milliseconds_to_beat_record(&self) -> f64 {
        let sqrt_part = self.quadratic_sqrt_part();

        let zero_1 = (self.time_limit_ms - sqrt_part) / 2.0;
        let zero_2 = (self.time_limit_ms + sqrt_part) / 2.0;

        let zero_1_fixed = (zero_1 + 1.0).floor();
        let zero_2_fixed = (zero_2 - 1.0).ceil();

        zero_2_fixed - zero_1_fixed + 1.0
    }

    fn quadratic_sqrt_part(&self) -> f64 {
        (self.time_limit_ms * self.time_limit_ms - 4.0 * self.distance_record_mm).sqrt()
    }
}

impl From<(f64, f64)> for Race {
    fn from(input: (f64, f64)) -> Self {
        Race {
            time_limit_ms: input.0,
            distance_record_mm: input.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_tuple() {
        let input = (30.0, 200.0);

        let expected = Race {
            time_limit_ms: 30.0,
            distance_record_mm: 200.0,
        };

        let result = Race::from(input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_number_of_pressing_milliseconds_to_beat_record() {
        let race = Race::from((30.0, 200.0));

        let expected = 9.0;

        let result = race.number_of_pressing_milliseconds_to_beat_record();

        assert!((result - expected).abs() < f64::EPSILON);
    }
}
