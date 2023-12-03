use std::collections::HashMap;

use crate::util::point_2d::Point2d;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Number {
    value: u32,
    row: usize,
    col_start: usize,
    col_end_exclusive: usize,
    is_part_number: bool,
}

impl Number {
    pub fn new(value: u32, row: usize, col_start: usize, col_end_exclusive: usize) -> Self {
        Number {
            value,
            row,
            col_start,
            col_end_exclusive,
            false,
        }
    }

    pub fn mark_as_part_number(&mut self) {
        self.is_part_number = true;
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Schematic {
    diagram: Vec<Vec<char>>,
    numbers: Vec<Number>,
    diagram_locations_to_numbers: HashMap<Point2d<i32>, u32>,
    diagram_locations_to_symbols: HashMap<Point2d<i32>, char>,
}

impl Schematic {
    pub fn new(input: &[String]) -> Self {
        let diagram = input.iter().map(|row| row.chars().collect()).collect();

        let diagram_locations_to_numbers = input
            .iter()
            .enumerate()
            .map(|(row_number, row)| Self::get_numbers_from_row_map(row_number, row))
            .flatten()
            .collect();

        let diagram_locations_to_symbols = input
            .iter()
            .enumerate()
            .map(|(row_number, row)| Self::get_symbols_from_row(row_number, row))
            .flatten()
            .collect();

        let numbers = input
            .iter()
            .enumerate()
            .map(|(row_number, row)| Self::get_numbers_from_row(row_number, row))
            .flatten()
            .collect();

        Schematic {
            diagram,
            numbers,
            diagram_locations_to_numbers,
            diagram_locations_to_symbols,
        }
    }

    pub fn get_part_numbers(&self) -> Vec<u32> {
        // vec of optional

        // get top optional
        // if some, add top optional
        // else, add top-left and top-right optional

        // add left and right optional

        // get bottom optional
        // if some, add bottom optional
        // else, add bottom-left and bottom-right optional

        // return filter of only Some
        unimplemented!()
    }

    fn get_numbers_from_row_map(row_number: usize, row: &str) -> HashMap<Point2d<i32>, u32> {
        let mut result = HashMap::new();
        let mut temp_value = 0;
        let mut temp_points = Vec::new();

        for (col_number, c) in row.chars().enumerate() {
            if c.is_digit(10) {
                temp_points.push(Point2d::new(row_number as i32, col_number as i32));
                temp_value = temp_value * 10 + c.to_digit(10).unwrap();
            } else if temp_value != 0 {
                for point in temp_points.drain(..) {
                    result.insert(point, temp_value);
                }

                temp_value = 0;
            }
        }

        result
    }

    fn get_numbers_from_row(row_number: usize, row: &str) -> Vec<Number> {
        let mut result = Vec::new();
        let mut temp_value = 0;
        let mut temp_col_start = None;

        for (col_number, c) in row.chars().enumerate() {
            if c.is_digit(10) {
                temp_col_start = temp_col_start.or(Some(col_number));

                temp_value = temp_value * 10 + c.to_digit(10).unwrap();
            } else if temp_value != 0 {
                let number =
                    Number::new(temp_value, row_number, temp_col_start.unwrap(), col_number);

                result.push(number);

                temp_col_start = None;
                temp_value = 0;
            }
        }

        result
    }

    fn get_symbols_from_row(row_number: usize, row: &str) -> HashMap<Point2d<i32>, char> {
        row.chars()
            .enumerate()
            .filter(|(_, c)| (*c != '.') && !c.is_digit(10))
            .map(|(col_number, c)| (Point2d::new(row_number as i32, col_number as i32), c))
            .collect()
    }

    fn get_surrounding_points(point: Point2d<i32>) -> Vec<Point2d<i32>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_from_row_map() {
        let row_number = 2;
        let row = "..35..633.";

        let expected = HashMap::from([
            (Point2d::new(2, 2), 35),
            (Point2d::new(2, 3), 35),
            (Point2d::new(2, 6), 633),
            (Point2d::new(2, 7), 633),
            (Point2d::new(2, 8), 633),
        ]);

        let result = Schematic::get_numbers_from_row_map(row_number, row);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_numbers_from_row() {
        let row_number = 2;
        let row = "..35..633.";

        let expected = vec![Number::new(35, 2, 2, 4), Number::new(633, 2, 6, 9)];

        let result = Schematic::get_numbers_from_row(row_number, row);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_symbols_from_row() {
        let row_number = 4;
        let row = "617*.....?";

        let expected = HashMap::from([(Point2d::new(4, 3), '*'), (Point2d::new(4, 9), '?')]);

        let result = Schematic::get_symbols_from_row(row_number, row);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new() {
        let input = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let expected_numbers = vec![
            Number::new(467, 0, 0, 3),
            Number::new(114, 0, 5, 8),
            Number::new(35, 2, 2, 4),
            Number::new(633, 2, 6, 9),
            Number::new(617, 4, 0, 3),
            Number::new(58, 5, 7, 9),
            Number::new(592, 6, 2, 5),
            Number::new(755, 7, 6, 9),
            Number::new(664, 9, 1, 4),
            Number::new(598, 9, 5, 8),
        ];

        let expected_numbers_map = HashMap::from([
            (Point2d::new(0, 0), 467),
            (Point2d::new(0, 1), 467),
            (Point2d::new(0, 2), 467),
            (Point2d::new(0, 5), 114),
            (Point2d::new(0, 6), 114),
            (Point2d::new(0, 7), 114),
            (Point2d::new(2, 2), 35),
            (Point2d::new(2, 3), 35),
            (Point2d::new(2, 6), 633),
            (Point2d::new(2, 7), 633),
            (Point2d::new(2, 8), 633),
            (Point2d::new(4, 0), 617),
            (Point2d::new(4, 1), 617),
            (Point2d::new(4, 2), 617),
            (Point2d::new(5, 7), 58),
            (Point2d::new(5, 8), 58),
            (Point2d::new(6, 2), 592),
            (Point2d::new(6, 3), 592),
            (Point2d::new(6, 4), 592),
            (Point2d::new(7, 6), 755),
            (Point2d::new(7, 7), 755),
            (Point2d::new(7, 8), 755),
            (Point2d::new(9, 1), 664),
            (Point2d::new(9, 2), 664),
            (Point2d::new(9, 3), 664),
            (Point2d::new(9, 5), 598),
            (Point2d::new(9, 6), 598),
            (Point2d::new(9, 7), 598),
        ]);

        let expected_symbols = HashMap::from([
            (Point2d::new(1, 3), '*'),
            (Point2d::new(3, 6), '#'),
            (Point2d::new(4, 3), '*'),
            (Point2d::new(5, 5), '+'),
            (Point2d::new(8, 3), '$'),
            (Point2d::new(8, 5), '*'),
        ]);

        let expected = Schematic {
            diagram: input.iter().map(|line| line.chars().collect()).collect(),
            numbers: expected_numbers,
            diagram_locations_to_numbers: expected_numbers_map,
            diagram_locations_to_symbols: expected_symbols,
        };

        let result = Schematic::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_part_numbers() {
        let input = [
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];

        let schematic = Schematic::new(&input);

        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];

        let result = schematic.get_part_numbers();

        assert_eq!(result, expected);
    }
}
