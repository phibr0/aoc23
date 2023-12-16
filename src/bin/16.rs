use rayon::{join, prelude::*};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

advent_of_code::solution!(16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn go(
    map: &Vec<Vec<char>>,
    visited: &Arc<Mutex<&mut HashSet<(usize, usize)>>>,
    tracers: &Arc<Mutex<&mut HashSet<(usize, usize, Direction)>>>,
    direction: Direction,
    (x, y): (usize, usize),
) {
    {
        visited.lock().unwrap().insert((x, y));

        if !tracers.lock().unwrap().insert((x, y, direction)) {
            return;
        }
    }

    match map[y][x] {
        '.' => match direction {
            Direction::Up => {
                if y > 0 {
                    go(&map, &visited, &tracers, direction, (x, y - 1));
                }
            }
            Direction::Left => {
                if x > 0 {
                    go(&map, &visited, &tracers, direction, (x - 1, y));
                }
            }
            Direction::Down => {
                if y < map.len() - 1 {
                    go(&map, &visited, &tracers, direction, (x, y + 1));
                }
            }
            Direction::Right => {
                if x < map[y].len() - 1 {
                    go(&map, &visited, &tracers, direction, (x + 1, y));
                }
            }
        },
        '/' => match direction {
            Direction::Up => {
                if x < map[y].len() - 1 {
                    go(&map, &visited, &tracers, Direction::Right, (x + 1, y));
                }
            }
            Direction::Left => {
                if y < map.len() - 1 {
                    go(&map, &visited, &tracers, Direction::Down, (x, y + 1));
                }
            }
            Direction::Down => {
                if x > 0 {
                    go(&map, &visited, &tracers, Direction::Left, (x - 1, y));
                }
            }
            Direction::Right => {
                if y > 0 {
                    go(&map, &visited, &tracers, Direction::Up, (x, y - 1));
                }
            }
        },
        '\\' => match direction {
            Direction::Up => {
                if x > 0 {
                    go(&map, &visited, &tracers, Direction::Left, (x - 1, y));
                }
            }
            Direction::Left => {
                if y > 0 {
                    go(&map, &visited, &tracers, Direction::Up, (x, y - 1));
                }
            }
            Direction::Down => {
                if x < map[y].len() - 1 {
                    go(&map, &visited, &tracers, Direction::Right, (x + 1, y));
                }
            }
            Direction::Right => {
                if y < map.len() - 1 {
                    go(&map, &visited, &tracers, Direction::Down, (x, y + 1));
                }
            }
        },
        '-' => match direction {
            Direction::Up => {
                join(
                    || {
                        if x > 0 {
                            go(&map, &visited, &tracers, Direction::Left, (x - 1, y));
                        }
                    },
                    || {
                        if x < map[y].len() - 1 {
                            go(&map, &visited, &tracers, Direction::Right, (x + 1, y));
                        }
                    },
                );
            }
            Direction::Left => {
                if x > 0 {
                    go(&map, &visited, &tracers, Direction::Left, (x - 1, y));
                }
            }
            Direction::Down => {
                join(
                    || {
                        if x > 0 {
                            go(&map, &visited, &tracers, Direction::Left, (x - 1, y));
                        }
                    },
                    || {
                        if x < map[y].len() - 1 {
                            go(&map, &visited, &tracers, Direction::Right, (x + 1, y));
                        }
                    },
                );
            }
            Direction::Right => {
                if x < map[y].len() - 1 {
                    go(&map, &visited, &tracers, Direction::Right, (x + 1, y));
                }
            }
        },
        '|' => {
            match direction {
                Direction::Up => {
                    if y > 0 {
                        go(&map, &visited, &tracers, Direction::Up, (x, y - 1));
                    }
                }
                Direction::Left => {
                    join(
                        || {
                            if y > 0 {
                                go(&map, &visited, &tracers, Direction::Up, (x, y - 1));
                            }
                        },
                        || {
                            if y < map.len() - 1 {
                                go(&map, &visited, &tracers, Direction::Down, (x, y + 1));
                            }
                        },
                    );
                }
                Direction::Down => {
                    if y < map.len() - 1 {
                        go(&map, &visited, &tracers, Direction::Down, (x, y + 1));
                    }
                }
                Direction::Right => {
                    join(
                        || {
                            if y > 0 {
                                go(&map, &visited, &tracers, Direction::Up, (x, y - 1));
                            }
                        },
                        || {
                            if y < map.len() - 1 {
                                go(&map, &visited, &tracers, Direction::Down, (x, y + 1));
                            }
                        },
                    );
                }
            };
        }
        _ => panic!("Unknown map tile"),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix: Vec<Vec<char>> = input
        .par_lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
    let shared_energized_tiles = Arc::new(Mutex::new(&mut energized_tiles));
    let mut tracers: HashSet<(usize, usize, Direction)> = HashSet::new();
    let shared_tracers = Arc::new(Mutex::new(&mut tracers));

    go(
        &matrix,
        &shared_energized_tiles,
        &shared_tracers,
        Direction::Right,
        (0, 0),
    );

    energized_tiles.len().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix: Vec<Vec<char>> = input
        .par_lines()
        .map(|line| line.chars().collect())
        .collect();

    let ((left, right), (top, bottom)) = join(
        || {
            join(
                || {
                    (0..matrix.len())
                        .par_bridge()
                        .map(|y| {
                            let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
                            let shared_energized_tiles = Arc::new(Mutex::new(&mut energized_tiles));
                            let mut tracers: HashSet<(usize, usize, Direction)> = HashSet::new();
                            let shared_tracers = Arc::new(Mutex::new(&mut tracers));

                            go(
                                &matrix,
                                &shared_energized_tiles,
                                &shared_tracers,
                                Direction::Right,
                                (0, y),
                            );

                            energized_tiles.len()
                        })
                        .max()
                        .unwrap()
                },
                || {
                    (0..matrix.len())
                        .par_bridge()
                        .map(|y| {
                            let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
                            let shared_energized_tiles = Arc::new(Mutex::new(&mut energized_tiles));
                            let mut tracers: HashSet<(usize, usize, Direction)> = HashSet::new();
                            let shared_tracers = Arc::new(Mutex::new(&mut tracers));

                            go(
                                &matrix,
                                &shared_energized_tiles,
                                &shared_tracers,
                                Direction::Left,
                                (matrix[y].len() - 1, y),
                            );

                            energized_tiles.len()
                        })
                        .max()
                        .unwrap()
                },
            )
        },
        || {
            join(
                || {
                    (0..matrix[0].len())
                        .par_bridge()
                        .map(|x| {
                            let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
                            let shared_energized_tiles = Arc::new(Mutex::new(&mut energized_tiles));
                            let mut tracers: HashSet<(usize, usize, Direction)> = HashSet::new();
                            let shared_tracers = Arc::new(Mutex::new(&mut tracers));

                            go(
                                &matrix,
                                &shared_energized_tiles,
                                &shared_tracers,
                                Direction::Down,
                                (x, 0),
                            );

                            energized_tiles.len()
                        })
                        .max()
                        .unwrap()
                },
                || {
                    (0..matrix[0].len())
                        .par_bridge()
                        .map(|x| {
                            let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();
                            let shared_energized_tiles = Arc::new(Mutex::new(&mut energized_tiles));
                            let mut tracers: HashSet<(usize, usize, Direction)> = HashSet::new();
                            let shared_tracers = Arc::new(Mutex::new(&mut tracers));

                            go(
                                &matrix,
                                &shared_energized_tiles,
                                &shared_tracers,
                                Direction::Up,
                                (x, matrix.len() - 1),
                            );

                            energized_tiles.len()
                        })
                        .max()
                        .unwrap()
                },
            )
        },
    );

    vec![left, right, top, bottom].into_iter().max().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
