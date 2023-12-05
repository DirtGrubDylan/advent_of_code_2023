mod almanac;

use crate::util::file_reader::to_string_vector;

pub fn run() {
    let _input = to_string_vector("inputs/day_5.txt").expect("Something went wrong with Day 5!");

    println!("Day 5 Part 1: {:?}", part_1());
    println!("Day 5 Part 2: {:?}", part_2());
}

fn part_1() -> u32 {
    unimplemented!()
}

fn part_2() -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let _input = to_string_vector("test_inputs/day_5.txt")
            .expect("Something went wrong with Day 5 Part 1 Test!");

        let expected = 35;

        let result = part_1();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let _input = to_string_vector("test_inputs/day_5.txt")
            .expect("Something went wrong with Day 5 Part 2 Test!");

        let expected = 0;

        let result = part_2();

        assert_eq!(result, expected);
    }
}
