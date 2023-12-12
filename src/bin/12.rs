use itertools::Itertools;
use rayon::join;
use std::collections::HashMap;
use std::str::FromStr;

advent_of_code::solution!(12);

#[derive(Debug, Clone, PartialEq, Hash)]
struct Entry {
    states: Vec<State>,
    ranges: Vec<usize>,
}

impl FromStr for Entry {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [system_states, state_durations] = s.split_whitespace().collect_vec()[0..2] else {
            return Err("Invalid entry");
        };
        let (states, ranges) = join(
            || {
                system_states
                    .chars()
                    .map(|c| State::from_str(&c.to_string()).unwrap())
                    .collect_vec()
            },
            || {
                state_durations
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec()
            },
        );
        Ok(Self { states, ranges })
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum State {
    Operational,
    Broken,
    Unknown,
}

impl FromStr for State {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Operational),
            "#" => Ok(Self::Broken),
            "?" => Ok(Self::Unknown),
            _ => Err("Invalid state"),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut memo = HashMap::new();
    input
        .lines()
        .map(|line| {
            let entry = Entry::from_str(line).unwrap();
            calculate_solutions(&entry.states, &entry.ranges, &mut memo)
        })
        .sum::<usize>()
        .into()
}

fn calculate_solutions(
    states: &[State],
    ranges: &[usize],
    memo: &mut HashMap<(Vec<State>, Vec<usize>), usize>,
) -> usize {
    if states.is_empty() {
        if ranges.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match states.first().unwrap() {
        State::Operational => calculate_solutions(&states[1..], ranges, memo),
        State::Broken => calculate_broken_solutions(states, ranges, memo),
        State::Unknown => {
            calculate_solutions(&states[1..], ranges, memo)
                + calculate_broken_solutions(states, ranges, memo)
        }
    }
}

fn calculate_broken_solutions(
    states: &[State],
    ranges: &[usize],
    memo: &mut HashMap<(Vec<State>, Vec<usize>), usize>,
) -> usize {
    if let Some(&result) = memo.get(&(states.to_vec(), ranges.to_vec())) {
        return result;
    }

    if ranges.is_empty() {
        return 0;
    }

    let duration = ranges[0];
    if states.len() < duration {
        return 0;
    }
    for i in 0..duration {
        if states[i] == State::Operational {
            return 0;
        }
    }
    if states.len() == duration {
        if ranges.len() == 1 {
            return 1;
        }
        return 0;
    }
    if states[duration] == State::Broken {
        return 0;
    }
    let result = calculate_solutions(&states[(duration + 1)..], &ranges[1..], memo);
    memo.insert((states.to_vec(), ranges.to_vec()), result);
    result
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut memo = HashMap::new();
    input
        .lines()
        .map(|line| {
            let mut entry = Entry::from_str(line).unwrap();
            entry.states = vec![
                entry.states.clone(),
                vec![State::Unknown],
                entry.states.clone(),
                vec![State::Unknown],
                entry.states.clone(),
                vec![State::Unknown],
                entry.states.clone(),
                vec![State::Unknown],
                entry.states.clone(),
            ]
            .concat();
            entry.ranges = vec![
                entry.ranges.clone(),
                entry.ranges.clone(),
                entry.ranges.clone(),
                entry.ranges.clone(),
                entry.ranges.clone(),
            ]
            .concat();
            calculate_solutions(&entry.states, &entry.ranges, &mut memo)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
