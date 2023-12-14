use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::min;

advent_of_code::solution!(13);

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_horizontal_reflection(matrix: &[Vec<char>]) -> Option<usize> {
    let mut possible_y_axes: Vec<usize> = vec![];
    for (i, (a, b)) in matrix.iter().tuple_windows::<(_, _)>().enumerate() {
        let is_same = a.iter().zip(b.iter()).all(|(a, b)| a == b);
        if is_same {
            possible_y_axes.push(i);
        }
    }

    for possible_y in possible_y_axes {
        let mut is_reflection = true;

        let mut a = &matrix[0..=possible_y];
        let mut b = &matrix[(possible_y + 1)..=(min((possible_y + 1) * 2, matrix.len() - 1))];

        if a.len() < b.len() {
            b = &b[0..a.len()];
        } else if b.len() < a.len() {
            a = &a[(a.len() - b.len())..];
        }

        for (a, b) in a.iter().zip(b.iter().rev()) {
            if a != b {
                is_reflection = false;
                break;
            }
        }

        if is_reflection {
            return Some(possible_y + 1);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .split("\n\n")
        .par_bridge()
        .map(|pattern| {
            let matrix = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            let h = find_horizontal_reflection(&matrix);
            let v = find_horizontal_reflection(&transpose(matrix));

            if h.is_some() {
                h.unwrap() * 100
            } else {
                v.unwrap_or(0)
            }
        })
        .sum::<usize>()
        .into()
}

fn find_horizontal_reflection_smudge(matrix: &[Vec<char>]) -> Option<usize> {
    let old_result = find_horizontal_reflection(&matrix);

    let mut possible_axes: Vec<usize> = vec![];
    for (i, (a, b)) in matrix.iter().tuple_windows::<(_, _)>().enumerate() {
        let diff_count = a.iter().zip(b.iter()).filter(|(a, b)| a != b).count();
        if diff_count <= 1 {
            possible_axes.push(i);
        }
    }

    for possible_y in possible_axes {
        let mut is_reflection = true;
        let mut smudge_found = false;

        let mut a = &matrix[0..=possible_y];
        let mut b = &matrix[(possible_y + 1)..=(min((possible_y + 1) * 2, matrix.len() - 1))];

        if a.len() < b.len() {
            b = &b[0..a.len()];
        } else if b.len() < a.len() {
            a = &a[(a.len() - b.len())..];
        }

        for (a, b) in a.iter().zip(b.iter().rev()) {
            if a != b {
                if smudge_found {
                    is_reflection = false;
                    break;
                } else {
                    smudge_found = true;
                }
            }
        }

        if is_reflection && smudge_found && old_result != Some(possible_y + 1) {
            return Some(possible_y);
        }
    }

    old_result
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .split("\n\n")
        .par_bridge()
        .map(|pattern| {
            let matrix = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            let h = find_horizontal_reflection_smudge(&matrix);
            let v = find_horizontal_reflection_smudge(&transpose(matrix));

            if h.is_some() {
                h.unwrap() * 100
            } else {
                v.unwrap_or(0)
            }
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
