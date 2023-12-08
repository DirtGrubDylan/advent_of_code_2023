use std::convert::From;
use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(input: char) -> Self {
        match input {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Cannot convert from '{input}' to Card!"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum ScoreType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
    score_type: ScoreType,
}

impl Hand {
    fn get_score_type(cards: &[Card]) -> ScoreType {
        let mut card_counter: [u8; 13] = [0; 13];

        for card in cards {
            if let Some(count) = card_counter.get_mut(*card as usize) {
                *count += 1;
            }
        }

        println!("{card_counter:?}");

        let mut five_of_a_kind = None;
        let mut four_of_a_kind = None;
        let mut full_house = None;
        let mut three_of_a_kind = None;
        let mut two_pair = None;
        let mut one_pair = None;
        let mut high_card = None;

        for (index, count) in card_counter.iter().enumerate() {
            let card = (index as u8) as Card;
        }

        unimplemented!()
    }

    fn get_cards(input: &str) -> Vec<Card> {
        input.chars().map(Card::from).collect()
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Cannot convert from 'y' to Card!")]
    fn test_card_from_char_panics() {
        let _card = Card::from('y');
    }

    #[test]
    fn test_hand_get_cards() {
        let input = "23456789TJQKA";

        let expected = vec![
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
            Card::Jack,
            Card::Queen,
            Card::King,
            Card::Ace,
        ];

        let result: Vec<Card> = Hand::get_cards(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_get_score_type() {
        let inputs = [
            "23456", "A23A4", "23432", "TTT98", "23332", "AA8AA", "AAAAA",
        ];

        let expected = vec![
            ScoreType::HighCard,
            ScoreType::OnePair,
            ScoreType::TwoPair,
            ScoreType::ThreeOfAKind,
            ScoreType::FullHouse,
            ScoreType::FourOfAKind,
            ScoreType::FiveOfAKind,
        ];

        let result: Vec<ScoreType> = inputs
            .into_iter()
            .map(Hand::get_cards)
            .map(|cards| Hand::get_score_type(&cards))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_from_str() {
        let input = "32T3K 765";

        let expected = Hand {
            cards: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
            bid: 765,
            score_type: ScoreType::TwoPair,
        };

        let result: Hand = input.parse().unwrap();

        assert_eq!(result, expected);
    }
}
