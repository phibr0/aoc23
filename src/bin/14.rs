use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

advent_of_code::solution!(14);

const ROTATION: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];
const CYCLE_COUNT: usize = 1_000; // seems to be enough

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Rock,
    Solid,
}

impl FromStr for Cell {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "O" => Ok(Cell::Rock),
            "#" => Ok(Cell::Solid),
            _ => Err("Invalid cell"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn compute_load(platform: &Vec<Vec<Cell>>) -> usize {
    let mut sum = 0;
    for y in 0..platform.len() {
        let load = platform.len() - y;
        for x in 0..platform[y].len() {
            if platform[y][x] == Cell::Rock {
                sum += load;
            }
        }
    }
    sum
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .par_lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<Cell>().unwrap())
                .collect_vec()
        })
        .collect()
}

fn find_space_in_direction(
    platform: &Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    direction: Direction,
) -> usize {
    let mut space = 0;
    match direction {
        Direction::North => {
            for y in (0..y).rev() {
                match platform[y][x] {
                    Cell::Empty => space += 1,
                    _ => break,
                }
            }
        }
        Direction::South => {
            for y in (y + 1)..platform.len() {
                match platform[y][x] {
                    Cell::Empty => space += 1,
                    _ => break,
                }
            }
        }
        Direction::East => {
            for x in (x + 1)..platform[y].len() {
                match platform[y][x] {
                    Cell::Empty => space += 1,
                    _ => break,
                }
            }
        }
        Direction::West => {
            for x in (0..x).rev() {
                match platform[y][x] {
                    Cell::Empty => space += 1,
                    _ => break,
                }
            }
        }
    }
    space
}

fn tilt_in_direction(platform: Vec<Vec<Cell>>, direction: Direction) -> Vec<Vec<Cell>> {
    let mut new_platform = platform.clone();

    match direction {
        Direction::North => {
            for (y, row) in platform.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == Cell::Rock {
                        let space = find_space_in_direction(&new_platform, x, y, direction);
                        if space > 0 {
                            new_platform[y - space][x] = Cell::Rock;
                            new_platform[y][x] = Cell::Empty;
                        }
                    }
                }
            }
        }
        Direction::South => {
            for (y, row) in platform.iter().enumerate().rev() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == Cell::Rock {
                        let space = find_space_in_direction(&new_platform, x, y, direction);
                        if space > 0 {
                            new_platform[y + space][x] = Cell::Rock;
                            new_platform[y][x] = Cell::Empty;
                        }
                    }
                }
            }
        }
        Direction::East => {
            for (y, row) in platform.iter().enumerate() {
                for (x, cell) in row.iter().enumerate().rev() {
                    if *cell == Cell::Rock {
                        let space = find_space_in_direction(&new_platform, x, y, direction);
                        if space > 0 {
                            new_platform[y][x + space] = Cell::Rock;
                            new_platform[y][x] = Cell::Empty;
                        }
                    }
                }
            }
        }
        Direction::West => {
            for (y, row) in platform.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if *cell == Cell::Rock {
                        let space = find_space_in_direction(&new_platform, x, y, direction);
                        if space > 0 {
                            new_platform[y][x - space] = Cell::Rock;
                            new_platform[y][x] = Cell::Empty;
                        }
                    }
                }
            }
        }
    }

    new_platform
}

pub fn part_one(input: &str) -> Option<usize> {
    let platform = parse_input(input);
    let platform = tilt_in_direction(platform, Direction::North);
    compute_load(&platform).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut platform = parse_input(input);

    for _ in 0..CYCLE_COUNT {
        for direction in ROTATION.iter() {
            platform = tilt_in_direction(platform, *direction);
        }
    }

    compute_load(&platform).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
