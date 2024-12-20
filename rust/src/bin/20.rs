use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;
use ndarray::{Array, Array1, Array2};

advent_of_code::solution!(20);

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Invalid tile"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse_input(input);

    let visited = solve_maze(&maze)?;

    let cheets = find_cheets(&maze, &visited);

    // dbg!(&cheets.iter().sorted().collect::<Vec<_>>());

    let cheet_count = cheets
        .iter()
        .filter(|(&time_saved, &_counter)| time_saved >= 100)
        .map(|(&_time_saved, &counter)| counter)
        .sum();

    Some(cheet_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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

fn solve_maze(maze: &Array2<Tile>) -> Option<Array2<u32>> {
    let shape = maze.dim();
    let start_position = find_tile(maze, Tile::Start)?;
    let mut visited = Array::from_elem(shape, u32::MAX);
    visited[start_position] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_position)));

    while let Some(Reverse((cost, position))) = queue.pop() {
        if maze[position] == Tile::End {
            return Some(visited);
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
    None
}

fn find_tile(maze: &Array2<Tile>, tile_type: Tile) -> Option<(usize, usize)> {
    maze.indexed_iter()
        .find(|(_, &tile)| tile == tile_type)
        .map(|(position, _tile)| position)
}

fn find_cheets(maze: &Array2<Tile>, visited: &Array2<u32>) -> HashMap<i32, u32> {
    let mut cheets = HashMap::new();

    let shape = maze.dim();
    let mut path = Vec::new();

    let mut current_position = find_tile(maze, Tile::Start).unwrap();
    let end_position = find_tile(maze, Tile::End).unwrap();

    while current_position != end_position {
        path.push(current_position);
        let current_section = visited[current_position];
        let mut next_path_coord = None;
        for direction in DIRECTIONS {
            let next_coordinates = next_position(current_position, direction, shape);

            match next_coordinates {
                Some(next_coord)
                    if maze[next_coord] == Tile::Empty
                        && visited[next_coord] == current_section + 1 =>
                {
                    next_path_coord = Some(next_coord);
                    continue;
                }
                Some(next_coord) if maze[next_coord] == Tile::Wall => {}
                _ => {
                    continue;
                }
            }

            let past_next_coordinates =
                next_position(current_position, (direction.0 * 2, direction.1 * 2), shape);

            let past_next_coordinates = match past_next_coordinates {
                Some(next_coord) if maze[next_coord] != Tile::Wall => next_coord,
                _ => continue,
            };

            let past_next_section = visited[past_next_coordinates];

            let time_saved = past_next_section as i32 - current_section as i32 - 2;

            if time_saved > 0 {
                cheets
                    .entry(time_saved)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
        match next_path_coord {
            Some(next) => current_position = next,
            None => break,
        }
    }
    cheets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
