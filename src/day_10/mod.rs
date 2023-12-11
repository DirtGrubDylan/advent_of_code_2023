mod pipes;

use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_10.txt").expect("Something went wrong with Day 10!");

    println!("Day 10 Part 1: {}", part_1(&input));
    println!("Day 10 Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    unimplemented!()
}

fn part_2(input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_10.txt")
            .expect("Something went wrong with Day 10 Part 1 Test!");

        let expected = 8;

        let result = part_1(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_10.txt")
            .expect("Something went wrong with Day 10 Part 2 Test!");

        let expected = 8;

        let result = part_2(&input);

        assert_eq!(result, expected);
    }
}
