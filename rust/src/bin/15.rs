use num_complex::Complex;

advent_of_code::solution!(15);

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Robot,
    Box,
    BoxL,
    BoxR,
    Empty,
}

const UP: Complex<i32> = Complex::new(0, -1);
const DOWN: Complex<i32> = Complex::new(0, 1);
const LEFT: Complex<i32> = Complex::new(-1, 0);
const RIGHT: Complex<i32> = Complex::new(1, 0);

fn parse_input(
    input: &str,
    map_parser: impl Fn(&str) -> (Vec<Vec<Tile>>, Complex<i32>),
) -> (Vec<Vec<Tile>>, Vec<Complex<i32>>, Complex<i32>) {
    let (map_raw, movements) = input.split_once("\n\n").unwrap();

    let (map, robot_pos) = map_parser(map_raw);

    let movements = movements
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => UP,
            'v' => DOWN,
            '<' => LEFT,
            '>' => RIGHT,
            _ => panic!("Invalid movement"),
        })
        .collect();

    (map, movements, robot_pos)
}

fn parse_map_regular(input: &str) -> (Vec<Vec<Tile>>, Complex<i32>) {
    let mut robot_pos = Complex::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '@' => {
                        robot_pos = Complex::new(x as i32, y as i32);
                        Tile::Robot
                    }
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect();

    (map, robot_pos)
}

fn parse_map_heavy(input: &str) -> (Vec<Vec<Tile>>, Complex<i32>) {
    let mut robot_pos = Complex::new(0, 0);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .flat_map(|(x, c)| match c {
                    '#' => [Tile::Wall, Tile::Wall],
                    '@' => {
                        robot_pos = Complex::new(x as i32 * 2, y as i32);
                        [Tile::Robot, Tile::Empty]
                    }
                    'O' => [Tile::BoxL, Tile::BoxR],
                    '.' => [Tile::Empty, Tile::Empty],
                    _ => panic!("Invalid tile"),
                })
                .collect()
        })
        .collect();
    (map, robot_pos)
}

fn try_move(map: &mut [Vec<Tile>], position: Complex<i32>, movement: Complex<i32>) -> bool {
    if can_move(map, position, movement) {
        move_self(map, position, movement);
        true
    } else {
        false
    }
}

fn can_move(map: &mut [Vec<Tile>], position: Complex<i32>, movement: Complex<i32>) -> bool {
    let next = position + movement;

    let next_object = &map[next.im as usize][next.re as usize];

    match (next_object, movement) {
        (Tile::Wall, _) => false,
        (Tile::Empty, _) => true,
        (Tile::BoxL, UP | DOWN) => {
            can_move(map, next, movement) && can_move(map, next + RIGHT, movement)
        }
        (Tile::BoxR, UP | DOWN) => {
            can_move(map, next, movement) && can_move(map, next + LEFT, movement)
        }
        (Tile::Box | Tile::BoxL | Tile::BoxR, _) => can_move(map, next, movement),
        (Tile::Robot, _) => panic!("Invalid tile; Only one robot allowed"),
    }
}

fn move_self(map: &mut [Vec<Tile>], position: Complex<i32>, movement: Complex<i32>) {
    let next = position + movement;

    let next_object = &map[next.im as usize][next.re as usize];

    match (next_object, movement) {
        (Tile::Wall, _) => panic!("Can't move into wall"),
        (Tile::Empty, _) => {
            move_to_empty(map, position, next);
        }
        (Tile::BoxL, UP | DOWN) => {
            move_self(map, next, movement);
            move_self(map, next + RIGHT, movement);
            move_to_empty(map, position, next);
        }
        (Tile::BoxR, UP | DOWN) => {
            move_self(map, next, movement);
            move_self(map, next + LEFT, movement);
            move_to_empty(map, position, next);
        }
        (Tile::Box | Tile::BoxL | Tile::BoxR, _) => {
            move_self(map, next, movement);
            move_to_empty(map, position, next);
        }
        (Tile::Robot, _) => panic!("Invalid tile; Only one robot allowed"),
    }
}

fn move_to_empty(map: &mut [Vec<Tile>], position: Complex<i32>, next: Complex<i32>) {
    map[next.im as usize][next.re as usize] = map[position.im as usize][position.re as usize];
    map[position.im as usize][position.re as usize] = Tile::Empty;
}

fn count_coordinates(map: &[Vec<Tile>]) -> u32 {
    let mut sum = 0;
    for (y, row) in map.iter().enumerate().skip(1) {
        for (x, tile) in row.iter().enumerate().skip(1) {
            if tile == &Tile::Box || tile == &Tile::BoxL {
                sum += y as u32 * 100 + x as u32;
            }
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, movements, mut robot_pos) = parse_input(input, parse_map_regular);

    for movement in movements {
        if try_move(&mut map, robot_pos, movement) {
            robot_pos += movement;
        }
    }

    Some(count_coordinates(&map))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, movements, mut robot_pos) = parse_input(input, parse_map_heavy);

    for movement in movements {
        if try_move(&mut map, robot_pos, movement) {
            robot_pos += movement;
        }
    }

    Some(count_coordinates(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
