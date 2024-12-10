use itertools::Itertools;
use num_complex::Complex;
use std::collections::HashMap;

advent_of_code::solution!(8);

fn is_in_bounds(coord: &Complex<i32>, width: i32, height: i32) -> bool {
    0 <= coord.re && coord.re < width && 0 <= coord.im && coord.im < height
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut positions_by_type = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                c => {
                    let coord = Complex::new(x as i32, y as i32);
                    positions_by_type.entry(c).or_insert(vec![]).push(coord);
                }
            }
        }
    }

    let width = input.lines().next()?.len() as i32;
    let height = input.lines().count() as i32;

    positions_by_type
        .values()
        .flat_map(|coords| coords.iter().permutations(2))
        .map(|coords| (coords[0], coords[1]))
        .map(|(coord1, coord2)| (*coord1 - *coord2) + *coord1)
        .filter(|antinode| is_in_bounds(antinode, width, height))
        .unique()
        .count()
        .try_into()
        .ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut positions_by_type = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                c => {
                    let coord = Complex::new(x as i32, y as i32);
                    positions_by_type.entry(c).or_insert(vec![]).push(coord);
                }
            }
        }
    }

    let width = input.lines().next()?.len() as i32;
    let height = input.lines().count() as i32;

    let is_in_bounds = |coord: &Complex<i32>| is_in_bounds(coord, width, height);

    positions_by_type
        .values()
        .flat_map(|coords| coords.iter().permutations(2))
        .map(|coords| (coords[0], coords[1]))
        .flat_map(|(coord1, coord2)| {
            let delta = *coord1 - *coord2;
            (0..)
                .map(|x| coord1 + delta * x)
                .take_while(&is_in_bounds)
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
