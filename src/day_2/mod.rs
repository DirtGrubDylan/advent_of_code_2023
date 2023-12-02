mod game;

use crate::util::file_reader::to_string_vector;
use game::{CubeCount, Game};

pub fn run() {
    let input = to_string_vector("inputs/day_2.txt").expect("Something went wrong with Day 2!");

    let games: Vec<Game> = input.iter().map(|line| line.parse().unwrap()).collect();

    println!("Day 2 Part 1: {:?}", part_1(&games));
    println!("Day 2 Part 2: {:?}", part_2(&games));
}

fn part_1(games: &[Game]) -> u32 {
    let part_1_count_limit = CubeCount::new(14, 13, 12);

    games
        .iter()
        .filter(|game| game.can_contain(part_1_count_limit))
        .map(|game| game.id)
        .sum()
}

fn part_2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| game.minimum_required_cubes.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_2.txt")
            .expect("Something went wrong with Day 2 Part 1 Test!");

        let games: Vec<Game> = input.iter().map(|line| line.parse().unwrap()).collect();

        let expected = 8;

        let result = part_1(&games);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_2.txt")
            .expect("Something went wrong with Day 2 Part 1 Test!");

        let games: Vec<Game> = input.iter().map(|line| line.parse().unwrap()).collect();

        let expected = 2_286;

        let result = part_2(&games);

        assert_eq!(result, expected);
    }
}
