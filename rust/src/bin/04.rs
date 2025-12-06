advent_of_code::solution!(4);

use ndarray::{Array1, Array2};

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Paper,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => panic!("Invalid tile"),
        }
    }
}

fn parse_input(input: &str) -> Array2<Tile> {
    let map: Array1<Tile> = input
        .lines()
        .flat_map(|line| line.trim().chars().map(Tile::from_char))
        .collect();

    let shape = (
        input.lines().count(),
        input.lines().next().unwrap().trim().len(),
    );

    map.into_shape_with_order(shape).unwrap()
}

fn get_next_position(
    position: Position,
    direction: (i8, i8),
    shape: (usize, usize),
) -> Option<Position> {
    let next_position = (
        position.0.checked_add_signed(direction.0 as isize)?,
        position.1.checked_add_signed(direction.1 as isize)?,
    );

    if next_position.0 < shape.0 && next_position.1 < shape.1 {
        Some(next_position)
    } else {
        None
    }
}

fn is_reachable(map: &Array2<Tile>, position: Position) -> bool {
    // count paper tiles around

    let papers_around = DIRECTIONS
        .iter()
        .filter(|&&direction| {
            if let Some(next) = get_next_position(position, direction, map.dim()) {
                map[next] == Tile::Paper
            } else {
                false
            }
        })
        .count();

    papers_around < 4
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let reachable_papers_count = map
        .indexed_iter()
        .filter(|&(_pos, &tile)| tile == Tile::Paper)
        .filter(|&(pos, _tile)| is_reachable(&map, pos))
        .count() as u32;

    Some(reachable_papers_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_input(input);

    let mut removed_papers = 0;

    loop {
        let mut to_remove = Vec::new();

        for (pos, &tile) in map.indexed_iter() {
            if tile == Tile::Paper && is_reachable(&map, pos) {
                to_remove.push(pos);
            }
        }

        if to_remove.is_empty() {
            break;
        }

        removed_papers += to_remove.len() as u32;

        for pos in to_remove {
            map[pos] = Tile::Empty;
        }
    }

    Some(removed_papers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
