use std::collections::{HashSet, VecDeque};

use num_complex::Complex;

advent_of_code::solution!(10);

const DIRECTIONS: [Complex<i32>; 4] = [
    Complex::new(0, 1),
    Complex::new(1, 0),
    Complex::new(0, -1),
    Complex::new(-1, 0),
];

struct Map {
    map: Vec<Vec<i16>>,
    range_x: std::ops::Range<i32>,
    range_y: std::ops::Range<i32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<i16>> = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i16)
                    .collect()
            })
            .collect();

        let range_x = 0..map[0].len() as i32;
        let range_y = 0..map.len() as i32;

        Self {
            map,
            range_x,
            range_y,
        }
    }

    fn get_altitude(&self, point: Complex<i32>) -> Option<i16> {
        self.map
            .get(point.im as usize)?
            .get(point.re as usize)
            .copied()
    }

    fn check_bounds(&self, point: &Complex<i32>) -> bool {
        self.range_x.contains(&point.re) && self.range_y.contains(&point.im)
    }
}

fn find_trails_exlude_visited(map: &Map, start: Complex<i32>) -> u32 {
    let mut counter = 0;
    let mut to_visit = VecDeque::from([start]);
    let mut visited = HashSet::new();

    while let Some(current) = to_visit.pop_back() {
        if !visited.insert(current) {
            continue;
        }

        let current_altitude = match map.get_altitude(current) {
            None => continue,
            Some(9) => {
                counter += 1;
                continue;
            }
            Some(altitude) => altitude,
        };

        DIRECTIONS
            .iter()
            .map(|d| current + d)
            .filter(|next| map.check_bounds(next))
            .filter(|next| map.get_altitude(*next).unwrap() == current_altitude + 1)
            .for_each(|next| to_visit.push_back(next));
    }
    counter
}

fn find_trails(map: &Map, start: Complex<i32>) -> u32 {
    let mut counter = 0;
    let mut to_visit = VecDeque::from([start]);

    while let Some(current) = to_visit.pop_back() {
        let current_altitude = match map.get_altitude(current) {
            None => continue,
            Some(9) => {
                counter += 1;
                continue;
            }
            Some(altitude) => altitude,
        };

        DIRECTIONS
            .iter()
            .map(|d| current + d)
            .filter(|next| map.check_bounds(next))
            .filter(|next| map.get_altitude(*next).unwrap() == current_altitude + 1)
            .for_each(|next| to_visit.push_back(next));
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let topographic_map = Map::new(input);

    let mut counter = 0;

    for (y, row) in topographic_map.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 0 {
                continue;
            }
            counter +=
                find_trails_exlude_visited(&topographic_map, Complex::new(x as i32, y as i32));
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topographic_map = Map::new(input);

    let mut counter = 0;

    for (y, row) in topographic_map.map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 0 {
                continue;
            }
            counter += find_trails(&topographic_map, Complex::new(x as i32, y as i32));
        }
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
