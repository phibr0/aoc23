use itertools::Itertools;
use rayon::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
enum CardType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialEq for CardType {
    fn eq(&self, other: &Self) -> bool {
        let self_value = match self {
            CardType::HighCard => 0,
            CardType::OnePair => 1,
            CardType::TwoPairs => 2,
            CardType::ThreeOfAKind => 3,
            CardType::FullHouse => 4,
            CardType::FourOfAKind => 5,
            CardType::FiveOfAKind => 6,
        };
        let other_value = match other {
            CardType::HighCard => 0,
            CardType::OnePair => 1,
            CardType::TwoPairs => 2,
            CardType::ThreeOfAKind => 3,
            CardType::FullHouse => 4,
            CardType::FourOfAKind => 5,
            CardType::FiveOfAKind => 6,
        };
        self_value == other_value
    }
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_value = match self {
            CardType::HighCard => 0,
            CardType::OnePair => 1,
            CardType::TwoPairs => 2,
            CardType::ThreeOfAKind => 3,
            CardType::FullHouse => 4,
            CardType::FourOfAKind => 5,
            CardType::FiveOfAKind => 6,
        };
        let other_value = match other {
            CardType::HighCard => 0,
            CardType::OnePair => 1,
            CardType::TwoPairs => 2,
            CardType::ThreeOfAKind => 3,
            CardType::FullHouse => 4,
            CardType::FourOfAKind => 5,
            CardType::FiveOfAKind => 6,
        };
        self_value.partial_cmp(&other_value)
    }
}

#[derive(Debug)]
struct Card {
    cards: Vec<char>,
    card_type: CardType,
    bid: u32,
}

impl Card {
    fn new(cards: Vec<char>, bid: u32) -> Self {
        if cards.len() != 5 {
            panic!("Invalid number of cards")
        }

        Self {
            card_type: Card::compute_card_type(&cards),
            cards,
            bid,
        }
    }

    fn new_two(cards: Vec<char>, bid: u32) -> Self {
        if cards.len() != 5 {
            panic!("Invalid number of cards")
        }

        let card_type = Card::compute_card_type_two(&cards);

        Self {
            card_type,
            cards,
            bid,
        }
    }

    fn compute_card_type_two(cards: &Vec<char>) -> CardType {
        if {
            let cards: &Vec<char> = &cards;
            let mut counts = HashMap::new();
            let mut joker_count = 0;
            for card in cards {
                if *card == 'J' {
                    joker_count += 1;
                } else {
                    let count = counts.entry(card).or_insert(0);
                    *count += 1;
                }
            }
            counts.values().any(|&count| count == 5)
                || (joker_count == 5)
                || (joker_count > 0 && counts.values().any(|&count| count == 5 - joker_count))
        } {
            return CardType::FiveOfAKind;
        } else if {
            let cards: &Vec<char> = &cards;
            let mut counts = HashMap::new();
            let mut joker_count = 0;
            for card in cards {
                if *card == 'J' {
                    joker_count += 1;
                } else {
                    let count = counts.entry(card).or_insert(0);
                    *count += 1;
                }
            }
            counts.values().any(|&count| count == 4)
                || (joker_count > 0 && counts.values().any(|&count| count == 4 - joker_count))
        } {
            return CardType::FourOfAKind;
        } else if Card::is_full_house(cards)
            || (Card::is_two_pairs(cards) && cards.iter().filter(|&p| *p == 'J').count() == 1)
        {
            return CardType::FullHouse;
        } else if {
            let cards: &Vec<char> = &cards;
            let mut counts = HashMap::new();
            let mut joker_count = 0;
            for card in cards {
                if *card == 'J' {
                    joker_count += 1;
                } else {
                    let count = counts.entry(card).or_insert(0);
                    *count += 1;
                }
            }
            counts.values().any(|&count| count == 3)
                || (joker_count > 0 && counts.values().any(|&count| count == 3 - joker_count))
        } {
            return CardType::ThreeOfAKind;
        } else if Card::is_two_pairs(cards) {
            return CardType::TwoPairs;
        } else if {
            let cards: &Vec<char> = &cards;
            let mut counts = HashMap::new();
            let mut joker_count = 0;
            for card in cards {
                if *card == 'J' {
                    joker_count += 1;
                } else {
                    let count = counts.entry(card).or_insert(0);
                    *count += 1;
                }
            }
            counts.values().any(|&count| count == 2)
                || (joker_count > 0 && counts.values().any(|&count| count == 2 - joker_count))
        } {
            return CardType::OnePair;
        } else {
            return CardType::HighCard;
        }
    }

    fn compute_card_type(cards: &Vec<char>) -> CardType {
        if Card::is_multiple_of_type(&cards, 5) {
            return CardType::FiveOfAKind;
        } else if Card::is_multiple_of_type(&cards, 4) {
            return CardType::FourOfAKind;
        } else if Card::is_full_house(&cards) {
            return CardType::FullHouse;
        } else if Card::is_multiple_of_type(&cards, 3) {
            return CardType::ThreeOfAKind;
        } else if Card::is_two_pairs(&cards) {
            return CardType::TwoPairs;
        } else if Card::is_multiple_of_type(&cards, 2) {
            return CardType::OnePair;
        } else {
            return CardType::HighCard;
        }
    }

    fn is_multiple_of_type(cards: &Vec<char>, card_count: u32) -> bool {
        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        counts.values().any(|&count| count == card_count)
    }

    fn is_full_house(cards: &Vec<char>) -> bool {
        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        counts.values().any(|&count| count == 3) && counts.values().any(|&count| count == 2)
    }

    fn is_two_pairs(cards: &Vec<char>) -> bool {
        let mut counts = HashMap::new();
        for card in cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        counts.values().filter(|&&count| count == 2).count() == 2
    }

    fn from_str(input: &str) -> Self {
        let (card, bid): (&str, &str) = input.split_whitespace().collect_tuple().unwrap();
        Self::new(card.chars().collect(), bid.parse().unwrap())
    }

    fn from_str_two(input: &str) -> Self {
        let (card, bid): (&str, &str) = input.split_whitespace().collect_tuple().unwrap();
        Self::new_two(card.chars().collect(), bid.parse().unwrap())
    }

    fn compare_char(a: &char, b: &char) -> Ordering {
        if a == b {
            return Ordering::Equal;
        }
        let a_value = match a {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => a.to_digit(10).unwrap(),
        };
        let b_value = match b {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => b.to_digit(10).unwrap(),
        };
        a_value.cmp(&b_value)
    }

    fn compare_char_two(a: &char, b: &char) -> Ordering {
        if a == b {
            return Ordering::Equal;
        }
        let a_value = match a {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'T' => 12,
            'J' => 2,
            _ => a.to_digit(10).unwrap() + 1,
        };
        let b_value = match b {
            'A' => 15,
            'K' => 14,
            'Q' => 13,
            'T' => 12,
            'J' => 2,
            _ => b.to_digit(10).unwrap() + 1,
        };
        a_value.cmp(&b_value)
    }

    fn compare(&self, other: &Card) -> Ordering {
        if self.card_type != other.card_type {
            return self.card_type.partial_cmp(&other.card_type).unwrap();
        } else {
            return Card::compare_char(
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|&(a, b)| a != b)
                    .unwrap()
                    .0,
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|&(a, b)| a != b)
                    .unwrap()
                    .1,
            );
        }
    }

    fn compare_two(&self, other: &Card) -> Ordering {
        if self.card_type != other.card_type {
            return self.card_type.partial_cmp(&other.card_type).unwrap();
        } else {
            return Card::compare_char_two(
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|&(a, b)| a != b)
                    .unwrap()
                    .0,
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .find(|&(a, b)| a != b)
                    .unwrap()
                    .1,
            );
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = input
        .par_lines()
        .map(|line| Card::from_str(line))
        .collect::<Vec<Card>>();

    cards.par_sort_by(Card::compare);

    let mut reduced: u32 = 0;
    for (i, card) in cards.iter().enumerate() {
        reduced += (i + 1) as u32 * card.bid;
    }
    reduced.into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = input
        .par_lines()
        .map(|line| Card::from_str_two(line))
        .collect::<Vec<Card>>();

    cards.par_sort_by(Card::compare_two);

    let mut reduced: u32 = 0;
    for (i, card) in cards.iter().enumerate() {
        reduced += (i + 1) as u32 * card.bid;
    }
    reduced.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(251224870));
    }
}
