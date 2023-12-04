mod schematic;

use crate::util::file_reader::to_string_vector;

use schematic::Schematic;

pub fn run() {
    let input = to_string_vector("inputs/day_3.txt").expect("Something went wrong with Day 3!");

    let schematic = Schematic::new(&input);

    println!("Day 3 Part 1: {:?}", part_1(&schematic));
    println!("Day 3 Part 2: {:?}", part_2(&schematic));
}

fn part_1(schematic: &Schematic) -> u32 {
    schematic.set_part_numbers().iter().sum()
}

fn part_2(schematic: &Schematic) -> u32 {
    schematic.get_gear_ratios().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_3.txt")
            .expect("Something went wrong with Day 3 Part 1 Test!");

        let schematic = Schematic::new(&input);

        let expected = 4_361;

        let result = part_1(&schematic);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_3.txt")
            .expect("Something went wrong with Day 3 Part 2 Test!");

        let schematic = Schematic::new(&input);

        let expected = 467_835;

        let result = part_2(&schematic);

        assert_eq!(result, expected);
    }
}
