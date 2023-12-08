use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Card {
    Joker,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ScoreType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    pub bid: u32,
    pub score_type: ScoreType,
    joker_rule: bool,
}

impl Hand {
    pub fn set_joker_rule(&mut self) {
        self.joker_rule = true;

        self.cards = self
            .cards
            .iter()
            .map(|card| match card {
                Card::Jack => Card::Joker,
                _ => *card,
            })
            .collect();

        self.score_type = Self::get_score_type(&self.cards);
    }

    fn get_score_type(cards: &[Card]) -> ScoreType {
        let mut card_counter = HashMap::new();

        for card in cards {
            card_counter
                .entry(card)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }

        let mut five_of_a_kind = None;
        let mut four_of_a_kind = None;
        let mut three_of_a_kind = None;
        let mut second_pair = None;
        let mut first_pair = None;
        let mut joker_count = 0;

        for (card, count) in card_counter {
            match count {
                _ if *card == Card::Joker => joker_count = count,
                5 => five_of_a_kind = Some(card),
                4 => four_of_a_kind = Some(card),
                3 => three_of_a_kind = Some(card),
                2 if first_pair.is_none() => first_pair = Some(card),
                2 if first_pair.is_some() => second_pair = Some(card),
                _ => continue,
            }
        }

        if four_of_a_kind.is_some() && (joker_count == 1) {
            ScoreType::FiveOfAKind
        } else if four_of_a_kind.is_some() {
            ScoreType::FourOfAKind
        } else if three_of_a_kind.is_some() && (joker_count == 2) {
            ScoreType::FiveOfAKind
        } else if three_of_a_kind.is_some() && (joker_count == 1) {
            ScoreType::FourOfAKind
        } else if three_of_a_kind.is_some() && first_pair.is_some() {
            ScoreType::FullHouse
        } else if three_of_a_kind.is_some() {
            ScoreType::ThreeOfAKind
        } else if second_pair.is_some() && (joker_count == 1) {
            ScoreType::FullHouse
        } else if second_pair.is_some() {
            ScoreType::TwoPair
        } else if first_pair.is_some() && (joker_count == 3) {
            ScoreType::FiveOfAKind
        } else if first_pair.is_some() && (joker_count == 2) {
            ScoreType::FourOfAKind
        } else if first_pair.is_some() && (joker_count == 1) {
            ScoreType::ThreeOfAKind
        } else if first_pair.is_some() {
            ScoreType::OnePair
        } else if five_of_a_kind.is_some() || (joker_count >= 4) {
            ScoreType::FiveOfAKind
        } else if joker_count == 3 {
            ScoreType::FourOfAKind
        } else if joker_count == 2 {
            ScoreType::ThreeOfAKind
        } else if joker_count == 1 {
            ScoreType::OnePair
        } else {
            ScoreType::HighCard
        }
    }

    fn get_cards(input: &str, joker_rule: bool) -> Vec<Card> {
        input
            .chars()
            .map(|c| match Card::from(c) {
                Card::Jack if joker_rule => Card::Joker,
                card => card,
            })
            .collect()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(
            self.joker_rule == other.joker_rule,
            "Both hands have to have the same joker rule"
        );

        let mut cmp = self.score_type.cmp(&other.score_type);

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if cmp.is_eq() {
                cmp = self_card.cmp(other_card);
            }
        }

        cmp
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = input.split_once(' ').unwrap();

        let cards = Self::get_cards(cards_str, false);
        let bid = bid_str.parse().unwrap();
        let score_type = Self::get_score_type(&cards);

        Ok(Hand {
            cards,
            bid,
            score_type,
            joker_rule: false,
        })
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

        let result: Vec<Card> = Hand::get_cards(input, false);

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
            .map(|input| Hand::get_cards(input, false))
            .map(|cards| Hand::get_score_type(&cards))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_get_score_type_with_joker_rule() {
        let inputs = [
            "32T3K", "T55J5", "KK677", "KTJJT", "QQQJA", "JTTQQ", "32TJK", "3JJJJ", "JJ556",
        ];

        let expected = vec![
            ScoreType::OnePair,
            ScoreType::FourOfAKind,
            ScoreType::TwoPair,
            ScoreType::FourOfAKind,
            ScoreType::FourOfAKind,
            ScoreType::FullHouse,
            ScoreType::OnePair,
            ScoreType::FiveOfAKind,
            ScoreType::FourOfAKind,
        ];

        let result: Vec<ScoreType> = inputs
            .into_iter()
            .map(|input| Hand::get_cards(input, true))
            .map(|cards| Hand::get_score_type(&cards))
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_from_str() {
        let input = "32T3K 765";

        let expected = Hand {
            cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
            bid: 765,
            score_type: ScoreType::OnePair,
            joker_rule: false,
        };

        let result: Hand = input.parse().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand_less_than_by_score_type() {
        let hand_1: Hand = "32T3K 765".parse().unwrap();
        let hand_2: Hand = "T55J5 684".parse().unwrap();

        assert!(hand_1 < hand_2);
    }

    #[test]
    fn test_hand_less_than_by_cards() {
        let hand_1: Hand = "KTJJT 220".parse().unwrap();
        let hand_2: Hand = "KK677 28".parse().unwrap();

        assert!(hand_1 < hand_2);
    }

    #[test]
    fn test_hand_less_than_by_score_type_with_joker_rule() {
        let mut hand_1: Hand = "KK677 28".parse().unwrap();
        let mut hand_2: Hand = "KTJJT 220".parse().unwrap();

        hand_1.set_joker_rule();
        hand_2.set_joker_rule();

        assert!(hand_1 < hand_2);
    }

    #[test]
    fn test_hand_less_than_by_cards_with_joker_rule() {
        let mut hand_1: Hand = "JKKK2 765".parse().unwrap();
        let mut hand_2: Hand = "2222Q 684".parse().unwrap();
        let mut hand_3: Hand = "JJJJJ 684".parse().unwrap();
        let mut hand_4: Hand = "JJJJ2 684".parse().unwrap();

        hand_1.set_joker_rule();
        hand_2.set_joker_rule();
        hand_3.set_joker_rule();
        hand_4.set_joker_rule();

        assert!(hand_1 < hand_2);
        assert!(hand_2 < hand_3);
        assert!(hand_2 < hand_3);
        assert!(hand_3 < hand_4);
        assert!(hand_2 < hand_4);
    }
}
