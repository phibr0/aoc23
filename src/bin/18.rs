#[macro_use]
extern crate lazy_static;
use colored::*;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashSet, str::FromStr};
advent_of_code::solution!(18);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err("Couldn't parse direction"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Task {
    direction: Direction,
    distance: usize,

    p2_direction: Direction,
    p2_distance: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Ground,
    Dug,
}

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?<direction>R|U|L|D) (?<distance>\d+) \(#(?<r>..)(?<g>..)(?<b>..)\)")
            .unwrap();
    static ref PART2_LINE_REGEX: Regex =
        Regex::new(r"(?<a>R|U|L|D) (?<b>\d+) \(#(?<distance>.....)(?<direction>.)\)").unwrap();
}

impl FromStr for Task {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = LINE_REGEX.captures(s).ok_or("Couldn't parse line")?;
        let direction = captures
            .name("direction")
            .ok_or("Couldn't parse direction")?
            .as_str()
            .parse::<Direction>()?;
        let distance = captures
            .name("distance")
            .ok_or("Couldn't parse distance")?
            .as_str()
            .parse::<usize>()
            .unwrap();

        let captures_part2 = PART2_LINE_REGEX.captures(s).ok_or("Couldn't parse line")?;
        let p2_direction_no = captures_part2
            .name("direction")
            .ok_or("Couldn't parse direction")?
            .as_str();
        let p2_direction = match p2_direction_no {
            "0" => Direction::Right,
            "2" => Direction::Left,
            "3" => Direction::Up,
            "1" => Direction::Down,
            _ => return Err("Couldn't parse direction"),
        };
        let p2_distance = usize::from_str_radix(
            captures_part2
                .name("distance")
                .ok_or("Couldn't parse distance")?
                .as_str(),
            16,
        )
        .unwrap();

        Ok(Task {
            direction,
            distance,
            p2_direction,
            p2_distance,
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tasks: Vec<Task> = input
        .par_lines()
        .map(Task::from_str)
        .map(Result::unwrap)
        .collect();

    let mut current_width: isize = 0;
    let mut min_width: isize = 0;
    let mut max_width: isize = 0;
    let mut current_height: isize = 0;
    let mut min_height: isize = 0;
    let mut max_height: isize = 0;

    for task in &tasks {
        match task.direction {
            Direction::Right => {
                current_width += task.distance as isize;
                if current_width > max_width {
                    max_width = current_width;
                }
            }
            Direction::Left => {
                current_width -= task.distance as isize;
                if current_width < min_width {
                    min_width = current_width;
                }
            }
            Direction::Up => {
                current_height += task.distance as isize;
                if current_height > max_height {
                    max_height = current_height;
                }
            }
            Direction::Down => {
                current_height -= task.distance as isize;
                if current_height < min_height {
                    min_height = current_height;
                }
            }
        }
    }

    let mut grid = vec![
        vec![Cell::Ground; ((max_width - min_width) + 1) as usize];
        ((max_height - min_height) + 1) as usize
    ];
    let mut current_position = (0 - min_width, 0 - min_height);

    for task in &tasks {
        match task.direction {
            Direction::Right => {
                for i in 0..task.distance {
                    let x = current_position.0 + i as isize;
                    let y = current_position.1;
                    grid[y as usize][x as usize] = Cell::Dug;
                }
                current_position.0 += task.distance as isize;
            }
            Direction::Left => {
                for i in 0..task.distance {
                    let x = current_position.0 - i as isize;
                    let y = current_position.1;
                    grid[y as usize][x as usize] = Cell::Dug;
                }
                current_position.0 -= task.distance as isize;
            }
            Direction::Up => {
                for i in 0..task.distance {
                    let x = current_position.0;
                    let y = current_position.1 + i as isize;
                    grid[y as usize][x as usize] = Cell::Dug;
                }
                current_position.1 += task.distance as isize;
            }
            Direction::Down => {
                for i in 0..task.distance {
                    let x = current_position.0;
                    let y = current_position.1 - i as isize;
                    grid[y as usize][x as usize] = Cell::Dug;
                }
                current_position.1 -= task.distance as isize;
            }
        }
    }

    let mut outside_set: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in grid.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == row.len() - 1 || y == grid.len() - 1 {
                if cell == Cell::Ground {
                    outside_set.insert((x, y));
                }
            }
        }
    }

    let mut inside_set: HashSet<(usize, usize)> = HashSet::new();
    for (y, row) in grid.clone().iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == Cell::Ground {
                inside_set.insert((x, y));
            }
        }
    }
    inside_set = inside_set.difference(&outside_set).cloned().collect();
    loop {
        let before_outside_count = outside_set.len();
        inside_set
            .difference(&outside_set.clone())
            .cloned()
            .for_each(|(x, y)| {
                if outside_set.contains(&(x + 1, y))
                    || outside_set.contains(&(x - 1, y))
                    || outside_set.contains(&(x, y + 1))
                    || outside_set.contains(&(x, y - 1))
                {
                    outside_set.insert((x, y));
                }
            });

        if before_outside_count == outside_set.len() {
            break;
        }
    }
    inside_set = inside_set.difference(&outside_set).cloned().collect();

    let mut count = 0;
    for row in grid {
        for cell in row {
            if let Cell::Dug = cell {
                count += 1;
            }
        }
    }

    (inside_set.len() + count).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let tasks: Vec<Task> = input
        .par_lines()
        .map(Task::from_str)
        .map(Result::unwrap)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
