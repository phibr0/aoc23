use colored::Colorize;
use rayon::prelude::*;
use std::{collections::HashSet, fmt::Display, str::FromStr};
advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Tile {
    VerticalPipe = b'|',
    HorizontalPipe = b'-',
    BendNorthEast = b'L',
    BendNorthWest = b'J',
    BendSouthWest = b'7',
    BendSouthEast = b'F',
    Ground = b'.',
    Start = b'S',
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Tile, ()> {
        match s {
            "|" => Ok(Tile::VerticalPipe),
            "-" => Ok(Tile::HorizontalPipe),
            "L" => Ok(Tile::BendNorthEast),
            "J" => Ok(Tile::BendNorthWest),
            "7" => Ok(Tile::BendSouthWest),
            "F" => Ok(Tile::BendSouthEast),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::VerticalPipe => "|",
            Tile::HorizontalPipe => "-",
            Tile::BendNorthEast => "L",
            Tile::BendNorthWest => "J",
            Tile::BendSouthWest => "7",
            Tile::BendSouthEast => "F",
            Tile::Ground => ".",
            Tile::Start => "S",
        };
        write!(f, "{}", c)
    }
}

fn find_s(matrix: &[Vec<Tile>]) -> (usize, usize) {
    for (y, row) in matrix.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col == &Tile::Start {
                return (x, y);
            }
        }
    }
    panic!("No S found");
}

#[derive(Debug)]
struct Surroundings {
    top: Option<Tile>,
    bottom: Option<Tile>,
    left: Option<Tile>,
    right: Option<Tile>,
}

fn surroundings(matrix: &[Vec<Tile>], (x, y): (usize, usize)) -> Surroundings {
    let top = if y == 0 { None } else { Some(matrix[y - 1][x]) };
    let bottom = if y == matrix.len() - 1 {
        None
    } else {
        Some(matrix[y + 1][x])
    };
    let left = if x == 0 { None } else { Some(matrix[y][x - 1]) };
    let right = if x == matrix[y].len() - 1 {
        None
    } else {
        Some(matrix[y][x + 1])
    };
    Surroundings {
        top,
        bottom,
        left,
        right,
    }
}

fn find_first_connection(
    matrix: &[Vec<Tile>],
    (x, y): (usize, usize),
    from: Option<(usize, usize)>,
) -> (usize, usize) {
    let surroundings = surroundings(matrix, (x, y));
    let current = matrix[y][x];
    match current {
        Tile::VerticalPipe => {
            if let Some(tile) = surroundings.top {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendSouthWest
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y - 1))
                {
                    return (x, y - 1);
                }
            }
            if let Some(tile) = surroundings.bottom {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendNorthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y + 1))
                {
                    return (x, y + 1);
                }
            }
        }
        Tile::HorizontalPipe => {
            if let Some(tile) = surroundings.left {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthEast
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x - 1))
                {
                    return (x - 1, y);
                }
            }
            if let Some(tile) = surroundings.right {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendSouthWest
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x + 1))
                {
                    return (x + 1, y);
                }
            }
        }
        Tile::BendNorthEast => {
            if let Some(tile) = surroundings.top {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendSouthWest
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y - 1))
                {
                    return (x, y - 1);
                }
            }
            if let Some(tile) = surroundings.right {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendSouthWest
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x + 1))
                {
                    return (x + 1, y);
                }
            }
        }
        Tile::BendNorthWest => {
            if let Some(tile) = surroundings.top {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendSouthWest
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y - 1))
                {
                    return (x, y - 1);
                }
            }
            if let Some(tile) = surroundings.left {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthEast
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x - 1))
                {
                    return (x - 1, y);
                }
            }
        }
        Tile::BendSouthWest => {
            if let Some(tile) = surroundings.bottom {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendNorthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y + 1))
                {
                    return (x, y + 1);
                }
            }
            if let Some(tile) = surroundings.left {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthEast
                    || tile == Tile::BendSouthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x - 1))
                {
                    return (x - 1, y);
                }
            }
        }
        Tile::BendSouthEast => {
            if let Some(tile) = surroundings.bottom {
                if (tile == Tile::VerticalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendNorthEast
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().1 != y + 1))
                {
                    return (x, y + 1);
                }
            }
            if let Some(tile) = surroundings.right {
                if (tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendSouthWest
                    || tile == Tile::Start)
                    && (from.is_none() || (from.unwrap().0 != x + 1))
                {
                    return (x + 1, y);
                }
            }
        }
        Tile::Start => {
            if let Some(tile) = surroundings.top {
                if tile == Tile::VerticalPipe
                    || tile == Tile::BendSouthWest
                    || tile == Tile::BendSouthEast
                {
                    return (x, y - 1);
                }
            }
            if let Some(tile) = surroundings.bottom {
                if tile == Tile::VerticalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendNorthEast
                {
                    return (x, y + 1);
                }
            }
            if let Some(tile) = surroundings.left {
                if tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthEast
                    || tile == Tile::BendSouthEast
                {
                    return (x - 1, y);
                }
            }
            if let Some(tile) = surroundings.right {
                if tile == Tile::HorizontalPipe
                    || tile == Tile::BendNorthWest
                    || tile == Tile::BendSouthWest
                {
                    return (x + 1, y);
                }
            }
        }
        _ => {}
    };

    print_surroundings(&surroundings, current);
    panic!("No connection found.")
}

fn print_surroundings(surroundings: &Surroundings, current: Tile) {
    println!(
        " {} \n{}{}{}\n {} ",
        surroundings.top.unwrap_or(Tile::Ground),
        surroundings.left.unwrap_or(Tile::Ground),
        current,
        surroundings.right.unwrap_or(Tile::Ground),
        surroundings.bottom.unwrap_or(Tile::Ground)
    );
}

fn print_path(matrix: &[Vec<Tile>], path: &[(usize, usize)]) {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if path.contains(&(x, y)) {
                print!("{}", matrix[y][x].to_string().blue().on_bright_cyan());
            } else {
                print!("{}", matrix[y][x]);
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = input
        .par_lines()
        .map(|line| {
            line.par_chars()
                .map(|char| {
                    char.to_string()
                        .parse::<Tile>()
                        .unwrap_or_else(|_| panic!("Invalid char: {}", char))
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<_>>();

    let initial_position = find_s(&matrix);
    let mut current_position = initial_position;
    let mut visited = vec![(0, 0), initial_position];
    let mut steps = 0;

    loop {
        let new_position = find_first_connection(
            &matrix,
            current_position,
            if visited.len() > 1 {
                Some(visited[visited.len() - 2])
            } else {
                None
            },
        );
        steps += 1;
        current_position = new_position;
        visited.push(current_position);

        if current_position == initial_position {
            break;
        }
    }

    if false {
        print_path(&matrix, &visited);
    }

    Some(steps / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let matrix = input
        .par_lines()
        .map(|line| {
            line.par_chars()
                .map(|char| {
                    char.to_string()
                        .parse::<Tile>()
                        .unwrap_or_else(|_| panic!("Invalid char: {}", char))
                })
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<_>>();

    let initial_position = find_s(&matrix);
    let mut current_position = initial_position;
    let mut visited: Vec<(usize, usize)> = vec![initial_position];

    loop {
        let new_position = find_first_connection(
            &matrix,
            current_position,
            if visited.len() > 1 {
                Some(visited[visited.len() - 2])
            } else {
                None
            },
        );
        current_position = new_position;
        visited.push(current_position);

        if current_position == initial_position {
            break;
        }
    }

    let mut position_set = HashSet::new();
    matrix.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, _)| {
            position_set.insert((j, i));
        })
    });
    let visited_set = visited.iter().cloned().collect::<HashSet<_>>();
    let not_visited_set = position_set
        .difference(&visited_set)
        .cloned()
        .collect::<HashSet<_>>();

    let mut outside_set = position_set
        .iter()
        .filter(|(x, y)| *x == 0 || *y == 0 || *x == matrix[0].len() - 1 || *y == matrix.len() - 1)
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&visited_set)
        .cloned()
        .collect::<HashSet<_>>();

    loop {
        let before_outside_count = outside_set.len();
        not_visited_set
            .difference(&outside_set.clone())
            .cloned()
            .for_each(|(x, y)| {
                if outside_set.contains(&(x + 1, y))
                    || outside_set.contains(&(x - 1, y))
                    || outside_set.contains(&(x, y + 1))
                    || outside_set.contains(&(x, y - 1))
                {
                    outside_set.insert((x, y));
                }
            });

        if before_outside_count == outside_set.len() {
            break;
        }
    }

    let inside_set = not_visited_set
        .difference(&outside_set)
        .cloned()
        .collect::<HashSet<_>>();

    let contained_set = inside_set
        .clone()
        .into_iter()
        .filter(|&(x, y)| {
            (0..x)
                .filter(|a| visited_set.get(&(*a, y)).is_some())
                .map(|a| visited_set.get(&(a, y)).unwrap())
                .filter(|(x, y)| {
                    matrix[*y][*x] == Tile::VerticalPipe
                        || matrix[*y][*x] == Tile::BendSouthEast
                        || matrix[*y][*x] == Tile::BendSouthWest
                })
                .count()
                % 2
                != 0
        })
        .collect::<HashSet<(usize, usize)>>();

    if true {
        print_path_2(
            &matrix,
            &visited_set,
            &inside_set,
            &contained_set,
            &outside_set,
        );
    }

    contained_set.len().into()
}

fn print_path_2(
    matrix: &[Vec<Tile>],
    path: &HashSet<(usize, usize)>,
    not_outside: &HashSet<(usize, usize)>,
    contained: &HashSet<(usize, usize)>,
    outside: &HashSet<(usize, usize)>,
) {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            if contained.contains(&(x, y)) {
                print!("{}", matrix[y][x].to_string().red().on_bright_red());
            } else if path.contains(&(x, y)) {
                print!("{}", matrix[y][x].to_string().blue().on_bright_cyan());
            } else if not_outside.contains(&(x, y)) {
                print!("{}", matrix[y][x].to_string().yellow().on_bright_yellow());
            } else if outside.contains(&(x, y)) {
                print!("{}", matrix[y][x].to_string().green().on_bright_green());
            } else {
                print!("{}", matrix[y][x]);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
