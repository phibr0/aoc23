use rayon::prelude::*;
use std::collections::VecDeque;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<usize> {
    let weights = input
        .par_lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut distance_map = vec![vec![usize::MAX; weights[0].len()]; weights.len()];
    distance_map[0][0] = 0;
    let mut queue: VecDeque<(usize, usize, Option<Direction>, usize)> =
        VecDeque::from(vec![(0, 0, None, 0)]);

    while !queue.is_empty() {
        let (x, y, direction, same_dir_step_count) = queue.pop_front().unwrap();
        let distance = distance_map[y][x];
        let weight = weights[y][x];

        if x > 0
            && distance_map[y][x - 1] > distance + weight
            && (direction != Some(Direction::Left) || same_dir_step_count < 2)
        {
            distance_map[y][x - 1] = distance + weight;
            queue.push_back((
                x - 1,
                y,
                Some(Direction::Left),
                if direction == Some(Direction::Left) {
                    same_dir_step_count + 1
                } else {
                    0
                },
            ));
        }
        if x < weights[0].len() - 1
            && distance_map[y][x + 1] > distance + weight
            && (direction != Some(Direction::Right) || same_dir_step_count < 2)
        {
            distance_map[y][x + 1] = distance + weight;
            queue.push_back((
                x + 1,
                y,
                Some(Direction::Right),
                if direction == Some(Direction::Right) {
                    same_dir_step_count + 1
                } else {
                    0
                },
            ));
        }
        if y > 0
            && distance_map[y - 1][x] > distance + weight
            && (direction != Some(Direction::Up) || same_dir_step_count < 2)
        {
            distance_map[y - 1][x] = distance + weight;
            queue.push_back((
                x,
                y - 1,
                Some(Direction::Up),
                if direction == Some(Direction::Up) {
                    same_dir_step_count + 1
                } else {
                    0
                },
            ));
        }
        if y < weights.len() - 1
            && distance_map[y + 1][x] > distance + weight
            && (direction != Some(Direction::Down) || same_dir_step_count < 2)
        {
            distance_map[y + 1][x] = distance + weight;
            queue.push_back((
                x,
                y + 1,
                Some(Direction::Down),
                if direction == Some(Direction::Down) {
                    same_dir_step_count + 1
                } else {
                    0
                },
            ));
        }
    }

    distance_map[weights.len() - 1][weights[0].len() - 1].into()
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
