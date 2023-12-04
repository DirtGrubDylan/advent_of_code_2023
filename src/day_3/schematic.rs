use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::util::point_2d::Point2d;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Number {
    pub value: u32,
    row: usize,
    col_start: usize,
    col_end_exclusive: usize,
    pub is_part_number: bool,
    pub related_gear_location: Option<Point2d<i32>>,
}

impl Number {
    pub fn new(value: u32, row: usize, col_start: usize, col_end_exclusive: usize) -> Self {
        Number {
            value,
            row,
            col_start,
            col_end_exclusive,
            is_part_number: false,
            related_gear_location: None,
        }
    }

    pub fn get_location_points(&self) -> Vec<Point2d<i32>> {
        (self.col_start..self.col_end_exclusive)
            .map(|col| {
                Point2d::new(
                    i32::try_from(self.row).unwrap(),
                    i32::try_from(col).unwrap(),
                )
            })
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Schematic {
    pub diagram: Vec<Vec<char>>,
    pub numbers: Vec<Rc<RefCell<Number>>>,
    pub diagram_locations_to_numbers: HashMap<Point2d<i32>, Rc<RefCell<Number>>>,
    pub diagram_locations_to_symbols: HashMap<Point2d<i32>, char>,
}

impl Schematic {
    pub fn new(input: &[String]) -> Self {
        let diagram = input.iter().map(|row| row.chars().collect()).collect();

        let diagram_locations_to_symbols = input
            .iter()
            .enumerate()
            .flat_map(|(row_number, row)| Self::get_symbols_from_row(row_number, row))
            .collect();

        let numbers: Vec<Rc<RefCell<Number>>> = input
            .iter()
            .enumerate()
            .flat_map(|(row_number, row)| Self::get_numbers_from_row(row_number, row))
            .map(|number| Rc::new(RefCell::new(number)))
            .collect();

        let mut diagram_locations_to_numbers = HashMap::new();

        for number_ref in &numbers {
            for point in number_ref.borrow().get_location_points() {
                diagram_locations_to_numbers.insert(point, Rc::clone(number_ref));
            }
        }

        Schematic {
            diagram,
            numbers,
            diagram_locations_to_numbers,
            diagram_locations_to_symbols,
        }
    }

    pub fn set_part_numbers(&self) -> Vec<u32> {
        let mut number_locations = HashMap::new();

        for number_ref in &self.numbers {
            for point in number_ref.borrow().get_location_points() {
                number_locations.insert(point, Rc::clone(number_ref));
            }
        }

        let symbol_surrounding_locations: Vec<Point2d<i32>> = self
            .diagram_locations_to_symbols
            .keys()
            .flat_map(|point| Self::get_surrounding_points(*point))
            .collect();

        for point in symbol_surrounding_locations {
            if let Some(number_ref) = self.diagram_locations_to_numbers.get(&point) {
                number_ref.borrow_mut().is_part_number = true;
            }
        }

        self.numbers
            .iter()
            .filter(|number_ref| number_ref.borrow().is_part_number)
            .map(|number_ref| number_ref.borrow().value)
            .collect()
    }

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        let gear_surrounding_locations: Vec<(Vec<Point2d<i32>>, Point2d<i32>)> = self
            .diagram_locations_to_symbols
            .iter()
            .filter(|(_, symbol)| **symbol == '*')
            .map(|(point, _)| (Self::get_surrounding_points(*point), *point))
            .collect();

        for (surrounding_points, gear_point) in gear_surrounding_locations {
            for point in surrounding_points {
                if let Some(number_ref) = self.diagram_locations_to_numbers.get(&point) {
                    number_ref.borrow_mut().related_gear_location = Some(gear_point);
                }
            }
        }

        let mut gear_values: HashMap<Point2d<i32>, Vec<u32>> = HashMap::new();

        for number_ref in &self.numbers {
            if let Some(point) = number_ref.borrow().related_gear_location {
                let values = gear_values.entry(point).or_default();

                values.push(number_ref.borrow().value);
            }
        }

        gear_values
            .values()
            .filter(|values| values.len() == 2)
            .map(|values| values.iter().product())
            .collect()
    }

    fn get_numbers_from_row(row_number: usize, row: &str) -> Vec<Number> {
        let mut result = Vec::new();
        let mut temp_value = 0;
        let mut temp_col_start = None;

        for (col_number, c) in row.chars().enumerate() {
            if c.is_ascii_digit() {
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

        if temp_value != 0 {
            let number = Number::new(temp_value, row_number, temp_col_start.unwrap(), row.len());

            result.push(number);
        }

        result
    }

    fn get_symbols_from_row(row_number: usize, row: &str) -> HashMap<Point2d<i32>, char> {
        row.chars()
            .enumerate()
            .filter(|(_, c)| (*c != '.') && !c.is_ascii_digit())
            .map(|(col_number, c)| {
                (
                    Point2d::new(
                        i32::try_from(row_number).unwrap(),
                        i32::try_from(col_number).unwrap(),
                    ),
                    c,
                )
            })
            .collect()
    }

    fn get_surrounding_points(point: Point2d<i32>) -> Vec<Point2d<i32>> {
        let top_left: (i32, i32) = (-1, -1);
        let top: (i32, i32) = (-1, 0);
        let top_right: (i32, i32) = (-1, 1);
        let left: (i32, i32) = (0, -1);
        let right: (i32, i32) = (0, 1);
        let bottom_left: (i32, i32) = (1, -1);
        let bottom: (i32, i32) = (1, 0);
        let bottom_right: (i32, i32) = (1, 1);

        vec![
            point.add_t(top_left),
            point.add_t(top),
            point.add_t(top_right),
            point.add_t(left),
            point.add_t(right),
            point.add_t(bottom_left),
            point.add_t(bottom),
            point.add_t(bottom_right),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_get_location_points() {
        let number = Number::new(467, 0, 0, 3);

        let expected = vec![Point2d::new(0, 0), Point2d::new(0, 1), Point2d::new(0, 2)];

        let result = number.get_location_points();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_get_surrounding_points() {
        let point = Point2d::new(0, 1);

        let expected = vec![
            Point2d::new(-1, 0),
            Point2d::new(-1, 1),
            Point2d::new(-1, 2),
            Point2d::new(0, 0),
            Point2d::new(0, 2),
            Point2d::new(1, 0),
            Point2d::new(1, 1),
            Point2d::new(1, 2),
        ];

        let result = Schematic::get_surrounding_points(point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_get_numbers_from_row() {
        let row_number = 2;
        let row = "..35..633.";

        let expected = vec![Number::new(35, 2, 2, 4), Number::new(633, 2, 6, 9)];

        let result = Schematic::get_numbers_from_row(row_number, row);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_get_symbols_from_row() {
        let row_number = 4;
        let row = "617*.....?";

        let expected = HashMap::from([(Point2d::new(4, 3), '*'), (Point2d::new(4, 9), '?')]);

        let result = Schematic::get_symbols_from_row(row_number, row);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_new() {
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
            Rc::new(RefCell::new(Number::new(467, 0, 0, 3))),
            Rc::new(RefCell::new(Number::new(114, 0, 5, 8))),
            Rc::new(RefCell::new(Number::new(35, 2, 2, 4))),
            Rc::new(RefCell::new(Number::new(633, 2, 6, 9))),
            Rc::new(RefCell::new(Number::new(617, 4, 0, 3))),
            Rc::new(RefCell::new(Number::new(58, 5, 7, 9))),
            Rc::new(RefCell::new(Number::new(592, 6, 2, 5))),
            Rc::new(RefCell::new(Number::new(755, 7, 6, 9))),
            Rc::new(RefCell::new(Number::new(664, 9, 1, 4))),
            Rc::new(RefCell::new(Number::new(598, 9, 5, 8))),
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

        let result = Schematic::new(&input);

        let result_numbers_map: HashMap<Point2d<i32>, u32> = result
            .diagram_locations_to_numbers
            .iter()
            .map(|(point, number_ref)| (*point, number_ref.borrow().value))
            .collect();

        assert_eq!(result.numbers, expected_numbers);
        assert_eq!(result.diagram_locations_to_symbols, expected_symbols);
        assert_eq!(result_numbers_map, expected_numbers_map);
    }

    #[test]
    fn test_schematic_set_part_numbers() {
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

        let result = schematic.set_part_numbers();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_set_part_numbers_2() {
        let input = ["...12".to_string(), "12*..".to_string()];

        let schematic = Schematic::new(&input);

        let expected = vec![12, 12];

        let result = schematic.set_part_numbers();

        println!("{schematic:?}");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_schematic_get_gear_ratios() {
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

        let expected = vec![16_345, 451_490];

        let mut result = schematic.get_gear_ratios();

        result.sort_unstable();

        assert_eq!(result, expected);
    }
}
