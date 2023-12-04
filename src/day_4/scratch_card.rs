use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct ScratchCard {
    pub id: u32,
    winning_numbers: HashSet<u32>,
    scratched_numbers: HashSet<u32>,
}

impl ScratchCard {
    pub fn points(&self) -> u32 {
        let number_of_winning_scratched: u32 = self
            .winning_numbers
            .intersection(&self.scratched_numbers)
            .count()
            .try_into()
            .unwrap();

        if number_of_winning_scratched == 0 {
            0
        } else {
            2_u32.pow(number_of_winning_scratched - 1)
        }
    }

    pub fn won_copies(&self) -> Vec<u32> {
        let number_of_winning_scratched: u32 = self
            .winning_numbers
            .intersection(&self.scratched_numbers)
            .count()
            .try_into()
            .unwrap();

        if number_of_winning_scratched == 0 {
            Vec::new()
        } else {
            let start = self.id + 1;
            let end_exclusive = start + number_of_winning_scratched;

            (start..end_exclusive).collect()
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl FromStr for ScratchCard {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (card_id_str, numbers_str) = input.split_once(": ").unwrap();

        let id: u32 = card_id_str
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse()
            .unwrap();

        let (winning_numbers, scratched_numbers) = numbers_str
            .split_once(" | ")
            .map(|(winning, scratched)| (to_u32_vec(winning), to_u32_vec(scratched)))
            .unwrap();

        Ok(ScratchCard {
            id,
            winning_numbers,
            scratched_numbers,
        })
    }
}

fn to_u32_vec(input: &str) -> HashSet<u32> {
    input
        .split(' ')
        .filter_map(|val| val.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "Card 111: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let expected = ScratchCard {
            id: 111,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            scratched_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        let result = input.parse::<ScratchCard>().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_points() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = vec![8, 2, 2, 1, 0, 0];

        let result: Vec<u32> = input
            .into_iter()
            .map(|row| row.parse::<ScratchCard>().unwrap())
            .map(|card| card.points())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_won_copies() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = vec![
            vec![2, 3, 4, 5],
            vec![3, 4],
            vec![4, 5],
            vec![5],
            Vec::new(),
            Vec::new(),
        ];

        let result: Vec<Vec<u32>> = input
            .into_iter()
            .map(|row| row.parse::<ScratchCard>().unwrap())
            .map(|card| card.won_copies())
            .collect();

        assert_eq!(result, expected);
    }
}
