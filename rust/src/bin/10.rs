use std::collections::VecDeque;

use num_complex::Complex;
use std::ops::Range;

advent_of_code::solution!(10);

const DIRECTIONS: [Complex<i32>; 4] = [
    Complex::new(0, 1),
    Complex::new(1, 0),
    Complex::new(0, -1),
    Complex::new(-1, 0),
];

struct Map {
    data: Vec<u8>,
    offset: usize,
    range_x: Range<i32>,
    range_y: Range<i32>,
}

impl Map {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.trim().lines().collect();
        let height = lines.len();
        let offset = lines[0].len();
        let mut data = Vec::with_capacity(offset * height);

        for line in lines {
            data.extend(line.chars().map(|c| c.to_digit(10).unwrap() as u8));
        }

        Self {
            data,
            offset,
            range_x: 0..offset.try_into().unwrap(),
            range_y: 0..height.try_into().unwrap(),
        }
    }

    fn get_altitude(&self, point: &Complex<i32>) -> Option<u8> {
        let index = self.point_to_index(point)?;
        Some(self.data[index])
    }

    fn check_bounds(&self, point: &Complex<i32>) -> bool {
        self.range_x.contains(&point.re) && self.range_y.contains(&point.im)
    }

    fn index_to_point(&self, index: usize) -> Complex<i32> {
        let x = index % self.offset;
        let y = index / self.offset;
        Complex::new(x as i32, y as i32)
    }

    fn point_to_index(&self, point: &Complex<i32>) -> Option<usize> {
        if !self.check_bounds(point) {
            return None;
        }
        let x = point.re as usize;
        let y = point.im as usize;
        let index = y * self.offset + x;
        Some(index)
    }
}

fn find_trails_exlude_visited(map: &Map, start: Complex<i32>) -> u32 {
    let mut counter = 0;
    let mut to_visit = VecDeque::from([start]);
    let mut visited = vec![false; map.data.len()];

    while let Some(current) = to_visit.pop_back() {
        let index = map.point_to_index(&current).unwrap();
        if visited[index] {
            continue;
        }
        visited[index] = true;

        let expected_altitude = 1 + match map.get_altitude(&current) {
            None => continue,
            Some(9) => {
                counter += 1;
                continue;
            }
            Some(altitude) => altitude,
        };

        for direction in DIRECTIONS.iter() {
            let next = current + direction;
            if let Some(altitude) = map.get_altitude(&next) {
                if altitude == expected_altitude {
                    to_visit.push_back(next);
                }
            }
        }
    }
    counter
}

fn find_trails(map: &Map, start: Complex<i32>) -> u32 {
    let mut counter = 0;
    let mut to_visit = VecDeque::from([start]);

    while let Some(current) = to_visit.pop_back() {
        let expected_altitude = map.get_altitude(&current).unwrap() + 1;

        for d in DIRECTIONS.iter() {
            let next = current + d;
            match map.get_altitude(&next) {
                Some(9) if 9 == expected_altitude => counter += 1,
                Some(altitude) if altitude == expected_altitude => to_visit.push_back(next),
                _ => (),
            }
        }
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    let topographic_map = Map::new(input);

    let counter = topographic_map
        .data
        .iter()
        .enumerate()
        .filter(|&(_, &altitude)| altitude == 0)
        .map(|(index, _)| topographic_map.index_to_point(index))
        .fold(0, |acc, start| {
            acc + find_trails_exlude_visited(&topographic_map, start)
        });

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let topographic_map = Map::new(input);

    let counter = topographic_map
        .data
        .iter()
        .enumerate()
        .filter(|&(_, &altitude)| altitude == 0)
        .map(|(index, _)| topographic_map.index_to_point(index))
        .fold(0, |acc, start| acc + find_trails(&topographic_map, start));

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
