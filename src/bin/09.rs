use rayon::prelude::*;
advent_of_code::solution!(9);

fn predict_next_diff(readings: &[isize]) -> isize {
    if readings.iter().all(|&n| n == 0) {
        return 0;
    }

    let differences: Vec<isize> = readings
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    predict_next_diff(&differences) + differences.last().unwrap()
}

fn predict_prev_diff(readings: &[isize]) -> isize {
    if readings.iter().all(|&n| n == 0) {
        return 0;
    }

    let differences: Vec<isize> = readings
        .windows(2)
        .map(|window| window[0] - window[1])
        .collect();

    predict_prev_diff(&differences) + differences.first().unwrap()
}

pub fn part_one(input: &str) -> Option<isize> {
    input
        .par_lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<isize>().expect("Failed to parse number"))
                .collect::<Vec<isize>>()
        })
        .map(|readings| readings.last().unwrap() + predict_next_diff(&readings))
        .sum::<isize>()
        .into()
}

pub fn part_two(input: &str) -> Option<isize> {
    input
        .par_lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<isize>().expect("Failed to parse number"))
                .collect::<Vec<isize>>()
        })
        .map(|readings| readings.first().unwrap() + predict_prev_diff(&readings))
        .sum::<isize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
