mod game;

use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_2.txt").expect("Something went wrong with Day 2!");

    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_2_part_1.txt")
            .expect("Something went wrong with Day 2 Part 1 Test!");

        unimplemented!();
    }

    #[test]
    fn test_part_2() {
        unimplemented!();
    }
}
