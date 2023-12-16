use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(15);

struct HolidayAsciiStringHelper {
    value: u32,
}

impl HolidayAsciiStringHelper {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, input: char) {
        self.value = (self.value + input as u32) * 17 % 256;
    }

    fn hash_from_str(input: &str) -> u32 {
        let mut helper = Self::new();
        input.chars().for_each(|c| helper.update(c));
        helper.value
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .replace("\n", "")
        .par_split(',')
        .map(HolidayAsciiStringHelper::hash_from_str)
        .sum::<u32>()
        .into()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut map: Vec<Vec<Lens>> = vec![vec![]; 256];

    let binding = input.replace("\n", "");
    binding.split(',').for_each(|sequence| {
        if sequence.contains("=") {
            let (key, value) = sequence.split('=').collect_tuple().unwrap();
            let lens_box = &map[HolidayAsciiStringHelper::hash_from_str(key) as usize];
            let lens = Lens {
                label: key,
                focal_length: value.parse().unwrap(),
            };
            let already_present = lens_box.iter().any(|lens| lens.label == key);
            if already_present {
                map[HolidayAsciiStringHelper::hash_from_str(key) as usize] = lens_box
                    .iter()
                    .map(|l| {
                        if l.label == key {
                            return lens.clone();
                        }
                        return l.clone();
                    })
                    .collect_vec();
            } else {
                map[HolidayAsciiStringHelper::hash_from_str(key) as usize].push(lens);
            }
        } else if sequence.contains('-') {
            let key = sequence.split('-').next().unwrap();
            let lens_box = &map[HolidayAsciiStringHelper::hash_from_str(key) as usize];
            map[HolidayAsciiStringHelper::hash_from_str(key) as usize] = lens_box
                .iter()
                .filter(|lens| lens.label != key)
                .map(|lens| lens.clone())
                .collect::<Vec<_>>()
        } else {
            panic!("Invalid sequence: {}", sequence);
        }
    });

    let mut sum = 0;
    for (i, lens_box) in map.iter().enumerate() {
        for (j, lens) in lens_box.iter().enumerate() {
            sum += (1 + i) * (1 + j) * lens.focal_length;
        }
    }
    sum.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
