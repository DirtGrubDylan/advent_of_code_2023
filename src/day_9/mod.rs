mod oasis;

use crate::util::file_reader::to_string_vector;

use oasis::Oasis;

pub fn run() {
    let input = to_string_vector("inputs/day_9.txt").expect("Something went wrong with Day 7!");

    let oasis = Oasis::new(&input);

    println!("Day 9 Part 1: {}", part_1(&oasis));
    println!("Day 9 Part 2: {}", part_2(&oasis));
}

fn part_1(oasis: &Oasis) -> i32 {
    oasis.sum_of_next_values()
}

fn part_2(oasis: &Oasis) -> i32 {
    oasis.sum_of_previous_values()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_9.txt")
            .expect("Something went wrong with Day 9 Part 1 Test!");

        let oasis = Oasis::new(&input);

        let expected = 114;

        let result = part_1(&oasis);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_9.txt")
            .expect("Something went wrong with Day 9 Part 2 Test!");

        let oasis = Oasis::new(&input);

        let expected = 2;

        let result = part_2(&oasis);

        assert_eq!(result, expected);
    }
}
