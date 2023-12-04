mod scratch_card;

use std::collections::HashMap;

use crate::util::file_reader::to_string_vector;

use scratch_card::ScratchCard;

pub fn run() {
    let input = to_string_vector("inputs/day_4.txt").expect("Something went wrong with Day 4!");

    let cards: Vec<ScratchCard> = input
        .iter()
        .map(|row| row.parse::<ScratchCard>().unwrap())
        .collect();

    println!("Day 4 Part 1: {:?}", part_1(&cards));
    println!("Day 4 Part 2: {:?}", part_2(&cards));
}

fn part_1(cards: &[ScratchCard]) -> u32 {
    cards.iter().map(ScratchCard::points).sum()
}

fn part_2(cards: &[ScratchCard]) -> u32 {
    let mut result = 0;
    let mut card_id_counter: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let id = card.id;

        let id_count = *card_id_counter
            .get(&id)
            .unwrap_or_else(|| panic!("Cannot find id: {id}"));

        result += id_count;

        for copied_id in card.won_copies() {
            card_id_counter
                .entry(copied_id)
                .and_modify(|count| *count += id_count);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_4.txt")
            .expect("Something went wrong with Day 4 Part 1 Test!");

        let cards: Vec<ScratchCard> = input
            .iter()
            .map(|row| row.parse::<ScratchCard>().unwrap())
            .collect();

        let expected = 13;

        let result = part_1(&cards);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_4.txt")
            .expect("Something went wrong with Day 4 Part 2 Test!");

        let cards: Vec<ScratchCard> = input
            .iter()
            .map(|row| row.parse::<ScratchCard>().unwrap())
            .collect();

        let expected = 30;

        let result = part_2(&cards);

        assert_eq!(result, expected);
    }
}
