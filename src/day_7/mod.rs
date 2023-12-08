mod card;

use crate::util::file_reader::to_string_vector;

use card::Hand;

pub fn run() {
    let input = to_string_vector("inputs/day_7.txt").expect("Something went wrong with Day 7!");

    let hands: Vec<Hand> = input.iter().filter_map(|row| row.parse().ok()).collect();

    println!("Day 7 Part 1: {}", part_1(&hands));
    println!("Day 7 Part 2: {}", part_2(&hands));
}

fn part_1(hands: &[Hand]) -> u32 {
    let mut hands_sorted = hands.to_vec();

    hands_sorted.sort();

    hands_sorted
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (u32::try_from(index + 1).unwrap(), hand.bid))
        .fold(0, |acc, (index, bid)| acc + index * bid)
}

fn part_2(hands: &[Hand]) -> u32 {
    let mut joker_rule_hands = hands.to_vec();

    joker_rule_hands.iter_mut().for_each(Hand::set_joker_rule);

    part_1(&joker_rule_hands)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_7.txt")
            .expect("Something went wrong with Day 7 Part 1 Test!");

        let hands: Vec<Hand> = input.iter().filter_map(|row| row.parse().ok()).collect();

        let expected = 6_440;

        let result = part_1(&hands);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_7.txt")
            .expect("Something went wrong with Day 7 Part 2 Test!");

        let hands: Vec<Hand> = input.iter().filter_map(|row| row.parse().ok()).collect();

        let expected = 5_905;

        let result = part_2(&hands);

        assert_eq!(result, expected);
    }
}
