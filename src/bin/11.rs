use rayon::prelude::*;
use std::collections::HashMap;
advent_of_code::solution!(11);

fn expand_empty_columns(grid: &mut Vec<Vec<char>>, replacement: char) {
    let mut empty_columns = Vec::new();
    for (i, _) in grid[0].iter().enumerate() {
        if grid
            .iter()
            .all(|row| row[i] == '.' || row[i] == replacement)
        {
            empty_columns.push(i);
        }
    }
    for i in empty_columns.iter().rev() {
        for row in grid.iter_mut() {
            row.insert(*i, replacement);
        }
    }
}

fn expand_empty_rows(grid: &mut Vec<Vec<char>>, replacement: char) {
    let mut empty_rows = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.' || c == replacement) {
            empty_rows.push(i);
        }
    }
    for i in empty_rows.iter().rev() {
        grid.insert(*i, vec![replacement; grid[0].len()]);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    expand_empty_columns(&mut grid, '.');
    expand_empty_rows(&mut grid, '.');

    let galaxies: Vec<(usize, usize)> = grid
        .par_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let result = galaxies
        .par_iter()
        .flat_map(|galaxy| {
            let galaxies = galaxies.clone();
            let other_galaxies: Vec<&(usize, usize)> =
                galaxies.iter().filter(|&g| g != galaxy).collect();
            let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
            let mut distance_map: Vec<Vec<usize>> =
                vec![vec![usize::MAX - 1; grid[0].len()]; grid.len()];
            let starting_point = (galaxy.0, galaxy.1);
            distance_map[starting_point.1][starting_point.0] = 0;
            let mut queue = vec![starting_point];

            while !queue.is_empty() {
                let current_point = queue.remove(0);
                let current_distance = distance_map[current_point.1][current_point.0];
                let mut adjacent_points: Vec<(usize, usize)> = Vec::new();
                if current_point.1 > 0 {
                    adjacent_points.push((current_point.0, current_point.1 - 1));
                }
                if current_point.0 < grid[0].len() - 1 {
                    adjacent_points.push((current_point.0 + 1, current_point.1));
                }
                if current_point.1 < grid.len() - 1 {
                    adjacent_points.push((current_point.0, current_point.1 + 1));
                }
                if current_point.0 > 0 {
                    adjacent_points.push((current_point.0 - 1, current_point.1));
                }

                for adjacent_point in adjacent_points {
                    if distance_map[adjacent_point.1][adjacent_point.0] == usize::MAX - 1 {
                        distance_map[adjacent_point.1][adjacent_point.0] = current_distance + 1;
                        queue.push(adjacent_point);
                    }
                }
            }

            for other_galaxy in other_galaxies {
                distances.insert(*other_galaxy, distance_map[other_galaxy.1][other_galaxy.0]);
            }

            distances.values().map(|&a| a).collect::<Vec<usize>>()
        })
        .sum::<usize>();

    Some(result / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    expand_empty_columns(&mut grid, 'x');
    expand_empty_rows(&mut grid, 'x');

    let galaxies: Vec<(usize, usize)> = grid
        .par_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let result = galaxies
        .par_iter()
        .flat_map(|galaxy| {
            let galaxies = galaxies.clone();
            let other_galaxies: Vec<&(usize, usize)> =
                galaxies.iter().filter(|&g| g != galaxy).collect();
            let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
            let mut distance_map: Vec<Vec<usize>> =
                vec![vec![usize::MAX - 1; grid[0].len()]; grid.len()];
            let starting_point = (galaxy.0, galaxy.1);
            distance_map[starting_point.1][starting_point.0] = 0;
            let mut queue = vec![starting_point];

            while !queue.is_empty() {
                let current_point = queue.remove(0);
                let current_distance = distance_map[current_point.1][current_point.0];
                let mut adjacent_points: Vec<(usize, usize)> = Vec::new();
                if current_point.1 > 0 {
                    adjacent_points.push((current_point.0, current_point.1 - 1));
                }
                if current_point.0 < grid[0].len() - 1 {
                    adjacent_points.push((current_point.0 + 1, current_point.1));
                }
                if current_point.1 < grid.len() - 1 {
                    adjacent_points.push((current_point.0, current_point.1 + 1));
                }
                if current_point.0 > 0 {
                    adjacent_points.push((current_point.0 - 1, current_point.1));
                }

                for adjacent_point in adjacent_points {
                    if distance_map[adjacent_point.1][adjacent_point.0] == usize::MAX - 1 {
                        let mut x = 1;
                        if grid[adjacent_point.1][adjacent_point.0] == 'x' {
                            x = 999_999;
                        }
                        distance_map[adjacent_point.1][adjacent_point.0] = current_distance + x;
                        queue.push(adjacent_point);
                    }
                }
            }

            for other_galaxy in other_galaxies {
                distances.insert(*other_galaxy, distance_map[other_galaxy.1][other_galaxy.0]);
            }

            distances.values().map(|&a| a).collect::<Vec<usize>>()
        })
        .sum::<usize>();

    Some(result / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
