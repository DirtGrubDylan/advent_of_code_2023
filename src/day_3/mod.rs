mod schematic;

use crate::util::file_reader::to_string_vector;

pub fn run() {
    let _input = to_string_vector("inputs/day_3.txt").expect("Something went wrong with Day 3!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let _input = to_string_vector("test_inputs/day_3.txt")
            .expect("Something went wrong with Day 3 Part 1 Test!");

        unimplemented!()
    }

    #[test]
    fn test_part_2() {
        let _input = to_string_vector("test_inputs/day_3.txt")
            .expect("Something went wrong with Day 3 Part 2 Test!");

        unimplemented!()
    }
}
