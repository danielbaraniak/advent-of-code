use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use ndarray::{Array, Array1, Array2};

advent_of_code::solution!(20);

const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const CHEAT_TIME_LIMIT: u8 = 20;
const MIN_CHEAT_SAVE: u32 = 100;

type Position = (usize, usize);

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

    let cheets = find_cheats(&maze, &visited);

    // dbg!(&cheets.iter().sorted().collect::<Vec<_>>());

    let cheet_count = cheets
        .iter()
        .filter(|(&time_saved, &_counter)| time_saved >= 100)
        .map(|(&_time_saved, &counter)| counter)
        .sum();

    Some(cheet_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = parse_input(input);

    let visited = solve_maze(&maze)?;

    let cheets = find_cheats_long(&maze, &visited);

    dbg!(&cheets.iter().sorted().collect::<Vec<_>>());

    let cheet_count = cheets
        .iter()
        .map(|(&_time_saved, &counter)| counter as u32)
        .sum();

    Some(cheet_count)
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
            if let Some(next_position) = get_next_position(position, direction, shape) {
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

fn find_tile(maze: &Array2<Tile>, tile_type: Tile) -> Option<Position> {
    maze.indexed_iter()
        .find(|(_, &tile)| tile == tile_type)
        .map(|(position, _tile)| position)
}

fn find_cheats(maze: &Array2<Tile>, visited: &Array2<u32>) -> HashMap<i32, u32> {
    let mut cheats = HashMap::new();

    let shape = maze.dim();

    let start_position = find_tile(maze, Tile::Start).unwrap();
    let end_position = find_tile(maze, Tile::End).unwrap();
    let race_path = get_path(visited, start_position, end_position);

    for current_position in race_path {
        let current_section = visited[current_position];
        for direction in DIRECTIONS {
            let next_coordinates = get_next_position(current_position, direction, shape);

            match next_coordinates {
                Some(next_coord) if maze[next_coord] == Tile::Wall => {}
                _ => {
                    continue;
                }
            }

            let past_next_coordinates =
                get_next_position(current_position, (direction.0 * 2, direction.1 * 2), shape);

            let past_next_coordinates = match past_next_coordinates {
                Some(next_coord) if maze[next_coord] != Tile::Wall => next_coord,
                _ => continue,
            };

            let past_next_section = visited[past_next_coordinates];

            let time_saved = past_next_section as i32 - current_section as i32 - 2;

            if time_saved > 0 {
                cheats
                    .entry(time_saved)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }
    cheats
}

fn get_path(visited: &Array2<u32>, start: Position, end: Position) -> Vec<Position> {
    let mut path: Vec<Position> = Vec::with_capacity(visited[end] as usize);
    path.push(start);

    let shape = visited.dim();

    let mut current_position = start;
    let mut current_section = visited[start];

    while current_position != end {
        for direction in DIRECTIONS {
            let next = get_next_position(current_position, direction, shape);
            match next {
                Some(next_coordinates) if visited[next_coordinates] == current_section + 1 => {
                    current_position = next_coordinates;
                    current_section += 1;
                    path.push(current_position);
                    break;
                }
                _ => {}
            }
        }
    }
    path
}

fn explore_neighbours(
    visited: &Array2<u32>,
    position: Position,
    available_exits: &mut HashSet<(u8, Position)>,
    steps_left: u8,
    locally_visited: &mut HashSet<Position>,
) {
    let segment_number = visited[position];
    if !locally_visited.insert(position) {
        return;
    }

    if segment_number != u32::MAX {
        available_exits.insert((steps_left, position));
    }

    if steps_left == 0 {
        return;
    }

    for direction in DIRECTIONS {
        let maybe_next = get_next_position(position, direction, visited.dim());
        if let Some(next) = maybe_next {
            explore_neighbours(
                visited,
                next,
                available_exits,
                steps_left - 1,
                locally_visited,
            )
        }
    }
}

fn find_cheats_long(maze: &Array2<Tile>, visited: &Array2<u32>) -> HashMap<i32, usize> {
    let mut cheats = HashMap::new();

    let shape = maze.dim();

    let start_position = find_tile(maze, Tile::Start).unwrap();
    let end_position = find_tile(maze, Tile::End).unwrap();
    let race_path = get_path(visited, start_position, end_position);

    for current_position in race_path {
        let current_section = visited[current_position];

        for direction in DIRECTIONS {
            let maybe_next = get_next_position(current_position, direction, shape);

            if let Some(next) = maybe_next {
                if maze[next] != Tile::Wall {
                    continue;
                }

                let mut available_exits: HashSet<(u8, Position)> = HashSet::new();
                let mut locally_visited = HashSet::new();
                explore_neighbours(
                    visited,
                    next,
                    &mut available_exits,
                    CHEAT_TIME_LIMIT - 1,
                    &mut locally_visited,
                );

                available_exits
                    .into_iter()
                    .map(|(step_left, position)| {
                        visited[position] as i32
                            - current_section as i32
                            - (CHEAT_TIME_LIMIT as i32 - step_left as i32)
                    })
                    .filter(|&cheat_save| cheat_save >= 50 as i32)
                    .counts_by(|saved_time| saved_time)
                    .iter()
                    .for_each(|(&saved_time, &counter)| {
                        cheats
                            .entry(saved_time)
                            .and_modify(|v| *v += counter)
                            .or_insert(counter);
                    });
            }
        }
    }
    cheats
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
