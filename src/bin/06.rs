use itertools::Itertools;
advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: usize,
    max_achieved_distance: usize,
}

fn parse_races(input: &str) -> Vec<Race> {
    let tmp = input
        .lines()
        .flat_map(|line| {
            line.split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();
    let mut races: Vec<Race> = Vec::new();
    for (i, race) in tmp.iter().enumerate() {
        if i == tmp.len() / 2 {
            break;
        }
        races.push(Race {
            time: *race,
            max_achieved_distance: tmp[i + tmp.len() / 2],
        });
    }
    races
}

pub fn part_one(input: &str) -> Option<usize> {
    let races = parse_races(input);

    races
        .iter()
        .map(|race| {
            let mut beats = 0;
            for (mm_per_ms, time) in (0..race.time).enumerate() {
                let remaining_time = race.time - time;
                if mm_per_ms * remaining_time > race.max_achieved_distance {
                    beats += 1;
                }
            }
            beats
        })
        .reduce(|a, b| a * b)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
pub fn part_two(input: &str) -> Option<usize> {
    let (race_time, distance): (usize, usize) = input
        .lines()
        .map(|line| {
            remove_whitespace(line.split(':').last().unwrap())
                .parse::<usize>()
                .expect("Couldn't parse input")
        })
        .collect_tuple()
        .expect("Invalid input");

    let mut beats = 0;
    for (mm_per_ms, time) in (0..race_time).enumerate() {
        let remaining_time = race_time - time;
        let possible_distance = mm_per_ms * remaining_time;
        if possible_distance > distance {
            beats += 1;
        }
    }

    beats.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
