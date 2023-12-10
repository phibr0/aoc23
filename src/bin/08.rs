use std::{collections::HashMap, str::Chars};

use num::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(8);

fn parse_directions(input: &str) -> (Chars, usize) {
    let tmp = input.lines().collect::<Vec<&str>>().first().unwrap().trim();
    (tmp.chars(), tmp.len())
}

fn parse_graph(input: &str) -> HashMap<&str, (&str, &str)> {
    let node_regex = regex::Regex::new(r"(?P<node>.+) = \((?P<left>.+), (?P<right>.+)\)").unwrap();
    input
        .lines()
        .skip(2)
        .map(|lines| {
            let captures = node_regex.captures(lines).unwrap();
            (
                captures.name("node").unwrap().as_str(),
                (
                    captures.name("left").unwrap().as_str(),
                    captures.name("right").unwrap().as_str(),
                ),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    const START: &str = "AAA";
    const END: &str = "ZZZ";

    let (directions, count_instructions) = parse_directions(input);
    let nodes = parse_graph(input);

    let mut required_steps = 0;
    let mut current_node = START;

    while current_node != END {
        let (left, right) = nodes.get(current_node).unwrap();

        match directions.clone().nth(required_steps % count_instructions) {
            Some('L') => current_node = left,
            Some('R') => current_node = right,
            _ => panic!("Invalid direction"),
        }

        required_steps += 1;
    }

    (required_steps as u32).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (directions, count_instructions) = parse_directions(input);
    let nodes = parse_graph(input);

    let current_nodes: Vec<&&str> = nodes.keys().filter(|key| key.ends_with('A')).collect();

    let depths_to_z: Vec<usize> = current_nodes
        .par_iter()
        .map(|node| {
            let mut current_node = *node;
            let mut depth = 0;

            while !current_node.ends_with('Z') {
                let (left, right) = nodes.get(current_node).unwrap();

                match directions.clone().nth(depth % count_instructions) {
                    Some('L') => current_node = left,
                    Some('R') => current_node = right,
                    _ => panic!("Invalid direction"),
                }

                depth += 1;
            }

            depth
        })
        .collect();

    depths_to_z
        .iter()
        .fold(1, |acc, depth| acc.lcm(depth))
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
