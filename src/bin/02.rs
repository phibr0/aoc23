use rayon::prelude::*;
use regex::Regex;
use std::str::FromStr;
advent_of_code::solution!(2);

#[derive(Debug)]
struct Game {
    id: usize,
    red: Vec<usize>,
    green: Vec<usize>,
    blue: Vec<usize>,
}
impl Game {
    fn extract_game_id(game: &str) -> Result<usize, &'static str> {
        let id_regex = Regex::new(r"Game (?P<id>\d+):").unwrap();

        let captures = id_regex.captures(game).ok_or("Failed to extract game ID")?;
        let id_str = captures
            .name("id")
            .ok_or("Failed to extract game ID")?
            .as_str();

        id_str.parse().map_err(|_| "Failed to parse game ID")
    }

    fn extract_cube_values(
        game: &str,
    ) -> Result<(Vec<usize>, Vec<usize>, Vec<usize>), &'static str> {
        let re = Regex::new(r"(?P<value>\d+) (?P<color>green|red|blue)").unwrap();
        let (mut red, mut green, mut blue) = (vec![], vec![], vec![]);

        re.captures_iter(game).for_each(|cap| {
            let color = cap.name("color").unwrap().as_str();
            let value = cap.name("value").unwrap().as_str().parse().unwrap();

            match color {
                "red" => red.push(value),
                "green" => green.push(value),
                "blue" => blue.push(value),
                _ => panic!("Unknown color: {}", color),
            }
        });

        Ok((red, green, blue))
    }

    fn is_valid(&self) -> bool {
        self.red.iter().max().unwrap() <= &12
            && self.green.iter().max().unwrap() <= &13
            && self.blue.iter().max().unwrap() <= &14
    }
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let id = Self::extract_game_id(game).unwrap();
        let (red, green, blue) = Self::extract_cube_values(game).unwrap();

        Ok(Game {
            id,
            red,
            green,
            blue,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .par_lines()
        .map(Game::from_str)
        .filter_map(Result::ok)
        .filter(Game::is_valid)
        .map(|game| game.id)
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .par_lines()
        .map(Game::from_str)
        .filter_map(Result::ok)
        .map(|game| {
            game.red.iter().max().unwrap()
                * game.green.iter().max().unwrap()
                * game.blue.iter().max().unwrap()
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
