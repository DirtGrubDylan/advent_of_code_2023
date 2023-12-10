mod map;

use crate::util::file_reader::to_string_vector;

use map::Map;

pub fn run() {
    let input = to_string_vector("inputs/day_8.txt").expect("Something went wrong with Day 8!");

    let map = Map::new(&input);

    println!("Day 8 Part 1: {}", part_1(&map));
    println!("Day 8 Part 2: {}", part_2(&map));
}

fn part_1(map: &Map) -> u32 {
    map.steps_between("AAA", "ZZZ")
}

fn part_2(map: &Map) -> u32 {
    map.steps_between_all('A', 'Z')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_8.txt")
            .expect("Something went wrong with Day 8 Part 1 Test!");

        let map = Map::new(&input);

        let expected = 6;

        let result = part_1(&map);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_8.txt")
            .expect("Something went wrong with Day 8 Part 2 Test!");

        let map = Map::new(&input);

        let expected = 6;

        let result = part_2(&map);

        assert_eq!(result, expected);
    }
}
