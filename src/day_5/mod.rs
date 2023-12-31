mod almanac;

use crate::util::file_reader::to_string_vector;

use almanac::Almanac;

pub fn run() {
    let input = to_string_vector("inputs/day_5.txt").expect("Something went wrong with Day 5!");

    let seeds = get_seed_ids(&input);

    let almanac = Almanac::new(&input[2..]);

    println!("Day 5 Part 1: {:?}", part_1(&seeds, &almanac));
    println!("Day 5 Part 2: {:?}", part_2(&seeds, &almanac));
}

fn get_seed_ids(input: &[String]) -> Vec<u64> {
    input[0]
        .split(' ')
        .skip(1)
        .map(|value| value.parse().unwrap())
        .collect()
}

fn part_1(seeds: &[u64], almanac: &Almanac) -> u64 {
    seeds
        .iter()
        .map(|&seed_id| almanac.seed_location(seed_id))
        .min()
        .unwrap()
}

fn part_2(seeds: &[u64], almanac: &Almanac) -> u64 {
    let seed_ranges: Vec<(u64, u64)> = seeds
        .chunks_exact(2)
        .filter(|chunk| chunk.len() == 2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    almanac.lowest_location_from_seed_ranges(&seed_ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_5.txt")
            .expect("Something went wrong with Day 5 Part 1 Test!");

        let seeds = get_seed_ids(&input);

        let almanac = Almanac::new(&input[2..]);

        let expected = 35;

        let result = part_1(&seeds, &almanac);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_5.txt")
            .expect("Something went wrong with Day 5 Part 2 Test!");

        let seeds = get_seed_ids(&input);

        let almanac = Almanac::new(&input[2..]);

        let expected = 46;

        let result = part_2(&seeds, &almanac);

        assert_eq!(result, expected);
    }
}
