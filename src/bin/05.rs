use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac {
    src_range: std::ops::Range<usize>,
    dest_start: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let initial_seeds = {
        let initial_seeds_regex = regex::Regex::new(r"seeds: (?P<seeds>(?:\d+\s*)+)").unwrap();
        initial_seeds_regex
            .captures(input)
            .unwrap()
            .name("seeds")
            .unwrap()
            .as_str()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };
    let map_regex =
        regex::Regex::new(r"(?m)(?P<from>.+)-to-(?P<to>.+) map:\n(?P<values>(?:\d+\s+)+)$")
            .unwrap();
    let mut translation_table = HashMap::new();
    let mut mapping_table: HashMap<&str, Vec<Almanac>> = HashMap::new();
    for capture in map_regex.captures_iter(input) {
        let from = capture.name("from").unwrap().as_str();
        let to = capture.name("to").unwrap().as_str();
        translation_table.insert(from, to);

        let values = capture
            .name("values")
            .unwrap()
            .as_str()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        values.iter().for_each(|(dest, src, len)| {
            let mapping = mapping_table.entry(to).or_insert_with(Vec::new);
            mapping.push(Almanac {
                dest_start: *dest,
                src_range: *src..(*src + len),
            });
        });
    }

    initial_seeds
        .par_iter()
        .map(|seed| {
            let mut step = "seed";
            let mut value = *seed;
            loop {
                let next_step = translation_table.get(step);
                if next_step.is_none() {
                    break;
                }
                let next_step = next_step.unwrap();

                let map = mapping_table.get(next_step).unwrap();

                value = map
                    .iter()
                    .find_map(|xmap| {
                        if xmap.src_range.contains(&value) {
                            Some(value - xmap.src_range.start + xmap.dest_start)
                        } else {
                            None
                        }
                    })
                    .or(Some(value))
                    .unwrap();

                step = next_step;
            }
            value
        })
        .min()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let initial_seeds: Vec<std::ops::Range<usize>> = {
        let initial_seeds_regex = regex::Regex::new(r"seeds: (?P<seeds>(?:\d+\s*)+)").unwrap();
        initial_seeds_regex
            .captures(input)
            .unwrap()
            .name("seeds")
            .unwrap()
            .as_str()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .tuples::<(usize, usize)>()
            .map(|(start, end)| (start..(start + end)))
            .collect()
    };
    let map_regex =
        regex::Regex::new(r"(?m)(?P<from>.+)-to-(?P<to>.+) map:\n(?P<values>(?:\d+\s+)+)$")
            .unwrap();
    let mut translation_table = HashMap::new();
    let mut mapping_table: HashMap<&str, Vec<Almanac>> = HashMap::new();
    for capture in map_regex.captures_iter(input) {
        let from = capture.name("from").unwrap().as_str();
        let to = capture.name("to").unwrap().as_str();
        translation_table.insert(from, to);

        let values = capture
            .name("values")
            .unwrap()
            .as_str()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_tuple::<(usize, usize, usize)>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        values.iter().for_each(|(dest, src, len)| {
            let mapping = mapping_table.entry(to).or_insert_with(Vec::new);
            mapping.push(Almanac {
                dest_start: *dest,
                src_range: *src..(*src + len),
            });
        });
    }

    let min_seed = initial_seeds
        .par_iter()
        .map(|range| {
            range
                .clone()
                .par_bridge()
                .map(|seed| {
                    let mut step = "seed";
                    let mut value = seed;
                    loop {
                        let next_step = translation_table.get(step);
                        if next_step.is_none() {
                            break;
                        }
                        let next_step = next_step.unwrap();

                        let map = mapping_table.get(next_step).unwrap();

                        value = map
                            .iter()
                            .find_map(|xmap| {
                                if xmap.src_range.contains(&value) {
                                    Some(value - xmap.src_range.start + xmap.dest_start)
                                } else {
                                    None
                                }
                            })
                            .or(Some(value))
                            .unwrap();

                        step = next_step;
                    }
                    (seed, value)
                })
                .min_by_key(|&(_, value)| value)
                .unwrap()
        })
        .min_by_key(|&(_, value)| value)
        .unwrap();

    min_seed.1.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
