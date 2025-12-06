use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use ndarray::{Array, Array1, Array2};

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_from(&self, position: &(usize, usize)) -> (usize, usize) {
        match self {
            Self::North => (position.0 - 1, position.1),
            Self::South => (position.0 + 1, position.1),
            Self::West => (position.0, position.1 - 1),
            Self::East => (position.0, position.1 + 1),
        }
    }

    fn get_value(&self) -> (i8, i8) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::West => (0, -1),
            Self::East => (0, 1),
        }
    }

    fn product(&self, other: &Self) -> i8 {
        let (x1, y1) = self.get_value();
        let (x2, y2) = other.get_value();
        x1 * x2 + y1 * y2
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

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

    let start_position = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == Tile::Start)
        .unwrap()
        .0;

    let (cost, _) = solve_maze(&maze, start_position);
    cost
}

pub fn part_two(input: &str) -> Option<u32> {
    let maze = parse_input(input);

    let start_position = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == Tile::Start)
        .unwrap()
        .0;

    let (_, visited) = solve_maze(&maze, start_position);
    let seat_count = count_best_paths(&maze, &visited);
    Some(seat_count)
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

fn solve_maze(maze: &Array2<Tile>, start_position: (usize, usize)) -> (Option<u32>, Array2<u32>) {
    let mut visited = Array::from_elem(maze.dim(), u32::MAX);

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, start_position, Direction::East)));

    while let Some(Reverse((cost, position, current_direction))) = queue.pop() {
        if maze[position] == Tile::End {
            return (Some(cost), visited);
        }
        for direction in DIRECTIONS.iter() {
            let turn_cost =
                (direction.product(&current_direction) - 1).unsigned_abs() as u32 * 1000;

            if turn_cost == 2000 {
                continue;
            }

            let next_position = direction.move_from(&position);
            let next_tile = maze[next_position];

            if next_tile == Tile::Wall {
                continue;
            }

            let next_cost = cost + turn_cost + 1;
            if next_cost < visited[next_position] {
                visited[next_position] = next_cost;
                queue.push(Reverse((next_cost, next_position, *direction)));
            }
        }
    }
    (None, visited)
}

fn count_best_paths(maze: &Array2<Tile>, costs: &Array2<u32>) -> u32 {
    let end_position = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == Tile::End)
        .unwrap()
        .0;

    let mut tile_count = 2;

    let mut queue = Vec::new();

    let end_cost = costs[end_position];
    let mut visited = HashSet::new();

    for direction in DIRECTIONS {
        let next = direction.move_from(&end_position);
        let next_cost = costs[next];
        if next_cost < end_cost {
            queue.push((next, direction));
            tile_count += 1;
            visited.insert(next);
        }
    }

    while let Some((position, current_direction)) = queue.pop() {
        if maze[position] == Tile::Start {
            continue;
        }
        for direction in DIRECTIONS {
            let product = direction.product(&current_direction);
            let next_position = direction.move_from(&position);

            if product == -1 || visited.contains(&next_position) {
                continue;
            }

            let current_cost = costs[position];
            let max_next_cost =
                current_cost + direction.product(&current_direction) as u32 * 1000 - 1;
            if costs[next_position] <= max_next_cost {
                queue.push((next_position, direction));
                tile_count += 1;
                visited.insert(next_position);
            }
        }
    }
    tile_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
