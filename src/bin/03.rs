use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
advent_of_code::solution!(3);

fn find_gear_ratio(input: &str, position: usize) -> Option<(u32, u32)> {
    let line_length = input.lines().next().unwrap().len() + 1;
    let mut checks = vec![];
    let is_first_line = position < line_length;
    let is_last_line = position > input.len() - line_length;
    let is_at_start = position % line_length == 0;
    let is_at_end = (position + 1) % line_length == 0;
    if !is_at_start {
        checks.push(position - 1);
        if !is_first_line {
            checks.push(position - line_length - 1);
        }
        if !is_last_line {
            checks.push(position + line_length - 1);
        }
    }
    if !is_first_line {
        checks.push(position - line_length);
    }
    if !is_last_line {
        checks.push(position + line_length);
    }
    if !is_at_end {
        checks.push(position + 1);
        if !is_first_line {
            checks.push(position - line_length + 1);
        }
        if !is_last_line {
            checks.push(position + line_length + 1);
        }
    }

    let ratios = checks
        .iter()
        .filter(|idx| input.chars().nth(**idx).unwrap().is_numeric())
        .collect::<Vec<_>>();

    if ratios.len() < 2 {
        return None;
    }

    Regex::new(r"\d+")
        .unwrap()
        .captures_iter(input)
        .filter(|cap| {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end() - 1;
            (start..=end).any(|idx| ratios.contains(&&idx))
        })
        .unique_by(|cap| cap.get(0).unwrap().start()..=cap.get(0).unwrap().end() - 1)
        .map(|cap| cap[0].parse::<u32>().unwrap())
        .collect_tuple::<(u32, u32)>()
}

fn is_valid_part_number(input: &str, start: usize, end: usize) -> bool {
    let line_length = input.lines().next().unwrap().len() + 1;
    !(start..=end)
        .flat_map(|idx| {
            let mut checks = vec![];
            let is_first_line = idx < line_length;
            let is_last_line = idx > input.len() - line_length;
            let is_at_start = idx % line_length == 0;
            let is_at_end = (idx + 1) % line_length == 0;
            if !is_at_start {
                checks.push(idx - 1);
                if !is_first_line {
                    checks.push(idx - line_length - 1);
                }
                if !is_last_line {
                    checks.push(idx + line_length - 1);
                }
            }
            if !is_first_line {
                checks.push(idx - line_length);
            }
            if !is_last_line {
                checks.push(idx + line_length);
            }
            if !is_at_end {
                checks.push(idx + 1);
                if !is_first_line {
                    checks.push(idx - line_length + 1);
                }
                if !is_last_line {
                    checks.push(idx + line_length + 1);
                }
            }
            checks
        })
        .filter(|idx| !(start..=end).contains(idx))
        .all(|idx| match input.chars().nth(idx) {
            Some('.') => true,
            Some('\n') => true,
            _ => false,
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    Regex::new(r"\d+")
        .unwrap()
        .captures_iter(input)
        .par_bridge()
        .map(|cap| {
            let number = cap[0].parse::<u32>().unwrap();
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end() - 1;
            let valid = is_valid_part_number(input, start, end);
            if valid {
                number
            } else {
                0
            }
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    Regex::new(r"\*+")
        .unwrap()
        .captures_iter(input)
        .par_bridge()
        .map(|cap| {
            let position = cap.get(0).unwrap().start();
            if let Some(ratio) = find_gear_ratio(input, position) {
                ratio.0 * ratio.1
            } else {
                0
            }
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
