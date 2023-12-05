use rayon::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn compute_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }

    fn compute_score(&self) -> u32 {
        let matches = self.compute_matches();

        if matches == 0 {
            return 0;
        }

        u32::pow(2, matches as u32 - 1)
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_regex = regex::Regex::new(r"Card\s+(?P<id>\d+):").unwrap();
        let id = id_regex
            .captures(s)
            .and_then(|c| c.name("id"))
            .and_then(|id| id.as_str().parse().ok())
            .ok_or("Failed to extract game ID")?;
        let winning_regex = regex::Regex::new(r": (?P<winning>(\d|\s)+) \|").unwrap();
        let winning_numbers = winning_regex
            .captures(s)
            .and_then(|c| c.name("winning"))
            .and_then(|winning| {
                winning
                    .as_str()
                    .split_whitespace()
                    .map(|n| n.parse().ok())
                    .collect::<Option<HashSet<u32>>>()
            })
            .ok_or("Failed to extract game ID")?;
        let numbers_regex = regex::Regex::new(r"\| (?P<numbers>(\d|\s)+)$").unwrap();
        let numbers = numbers_regex
            .captures(s)
            .and_then(|c| c.name("numbers"))
            .and_then(|numbers| {
                numbers
                    .as_str()
                    .split_whitespace()
                    .map(|n| n.parse().ok())
                    .collect::<Option<Vec<u32>>>()
            })
            .ok_or("Failed to extract game ID")?;

        Ok(Self {
            id,
            winning_numbers,
            numbers,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .par_lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .map(|card| card.compute_score())
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .par_lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut queue = VecDeque::<&Card>::from(cards.iter().collect::<Vec<_>>());
    let mut card_count: u32 = cards.len() as u32;
    while queue.len() > 0 {
        let card = queue.pop_front().unwrap();

        let score = card.compute_matches();
        card_count += score as u32;

        if score == 0 {
            continue;
        }

        let range = (card.id)..(card.id + score);
        queue.extend(cards[range].iter());
    }

    card_count.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
