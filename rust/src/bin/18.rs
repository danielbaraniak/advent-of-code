use std::{cmp::Reverse, collections::BinaryHeap};

use ndarray::{Array, Array2};

advent_of_code::solution!(18);

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const GRID_DIMENSIONS: (usize, usize) = (7, 7);
// const GRID_DIMENSIONS: (usize, usize) = (71, 71);
const START_POSITION: (usize, usize) = (0, 0);

pub fn part_one(input: &str) -> Option<u32> {
    let length = 12;
    // let length = 1024;

    let byte_positions: Vec<(usize, usize)> = input
        .lines()
        .take(length)
        .map(|line| line.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let map = create_grid(GRID_DIMENSIONS, byte_positions.iter());
    let end_position = (GRID_DIMENSIONS.0 - 1, GRID_DIMENSIONS.1 - 1);

    let (cost, _) = solve_maze(&map, START_POSITION, end_position);
    cost
}

pub fn part_two(input: &str) -> Option<String> {
    let end_position = (GRID_DIMENSIONS.0 - 1, GRID_DIMENSIONS.1 - 1);
    let byte_positions: Vec<(usize, usize)> = input
        .lines()
        .map(|line| line.trim().split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let byte_number = (0..byte_positions.len())
        .collect::<Vec<usize>>()
        .partition_point(|i| {
            let map = create_grid(GRID_DIMENSIONS, byte_positions.iter().take(*i));
            let (cost, _) = solve_maze(&map, START_POSITION, end_position);
            cost.is_some()
        })
        - 1;

    let result: String = byte_positions[byte_number].0.to_string()
        + ","
        + &byte_positions[byte_number].1.to_string();
    Some(result)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

fn create_grid<'a>(
    shape: (usize, usize),
    byte_positions: impl Iterator<Item = &'a (usize, usize)>,
) -> Array2<Tile> {
    let mut map = Array2::from_elem(shape, Tile::Empty);

    byte_positions.for_each(|position| map[*position] = Tile::Wall);
    map
}

fn next_position(
    position: (usize, usize),
    direction: (i8, i8),
    shape: (usize, usize),
) -> Option<(usize, usize)> {
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

fn solve_maze(
    maze: &Array2<Tile>,
    start_position: (usize, usize),
    end_position: (usize, usize),
) -> (Option<u32>, Array2<u32>) {
    let shape = maze.dim();
    let mut visited = Array::from_elem(shape, u32::MAX);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_position)));

    while let Some(Reverse((cost, position))) = queue.pop() {
        if position == end_position {
            return (Some(cost), visited);
        }
        for direction in DIRECTIONS {
            if let Some(next_position) = next_position(position, direction, shape) {
                let next_tile = maze[next_position];

                if next_tile == Tile::Wall {
                    continue;
                }

                let next_cost = cost + 1;
                if next_cost < visited[next_position] {
                    visited[next_position] = next_cost;
                    queue.push(Reverse((next_cost, next_position)));
                }
            }
        }
    }
    (None, visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
