use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    input
        .trim()
        .lines()
        .map(|calibration| {
            let chars = calibration
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>();

            format!("{}{}", chars[0], chars[chars.len() - 1])
                .parse::<usize>()
                .expect(calibration)
        })
        .sum::<usize>()
        .into()
}

const PATTERNS: [(&str, &str); 18] = [
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
    ("1", "1"),
    ("2", "2"),
    ("3", "3"),
    ("4", "4"),
    ("5", "5"),
    ("6", "6"),
    ("7", "7"),
    ("8", "8"),
    ("9", "9"),
];
pub fn part_two(input: &str) -> Option<usize> {
    input
        .trim()
        .par_lines()
        .map(|c| {
            let indices = PATTERNS
                .iter()
                .flat_map(|pattern| {
                    c.match_indices(&pattern.0)
                        .map(|(index, _)| index)
                        .map(|index| (index, pattern.1))
                })
                .sorted_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b))
                .collect::<Vec<_>>();

            format!("{}{}", indices[0].1, indices[indices.len() - 1].1)
                .parse::<usize>()
                .expect(c)
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
        assert_eq!(result, Some(77));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
