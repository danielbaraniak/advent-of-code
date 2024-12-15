use std::str::FromStr;

use lazy_static::lazy_static;
use num_complex::Complex;
use regex::Regex;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    let width = 11;
    let height = 7;
    let mut safety_factor = [0; 4];
    let quadrants = [
        (0..height / 2, 0..width / 2),
        (0..height / 2, div_ceil(width, 2)..width),
        (div_ceil(height, 2)..height, 0..width / 2),
        (div_ceil(height, 2)..height, div_ceil(width, 2)..width),
    ];

    let which_quadrant = |position: Complex<i32>| -> Option<usize> {
        for (i, (y_range, x_range)) in quadrants.iter().enumerate() {
            if y_range.contains(&position.im) && x_range.contains(&position.re) {
                return Some(i);
            }
        }
        None
    };

    robots
        .iter()
        .map(|robot| robot.position_at_time(100, width, height))
        .for_each(|position| {
            if let Some(quadrant) = which_quadrant(position) {
                safety_factor[quadrant] += 1;
            }
        });

    let total = safety_factor.iter().product::<u32>();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    let width = 101;
    let height = 103;

    let mut i = 0;

    let mut input = String::new();

    loop {
        robots
            .iter_mut()
            .for_each(|robot| robot.move_step(width, height));
        i += 1;
        let map = display_grid(&robots, width, height);

        if detect_image(&map) {
            println!("{}", map);
            println!("{:>0width$}", i, width = width as usize);

            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
}

const PATTERN: &str = r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)";

lazy_static! {
    static ref game_regex: Regex = Regex::new(PATTERN).unwrap();
}
#[derive(Debug)]
struct Robot {
    start_position: Complex<i32>,
    current_position: Complex<i32>,
    velocity: Complex<i32>,
}

#[derive(Debug)]
enum RobotError {
    ParsingError,
}

impl FromStr for Robot {
    type Err = RobotError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captured_numbers = game_regex
            .captures(s)
            .ok_or(RobotError::ParsingError)?
            .iter()
            .skip(1)
            .map(|capture| {
                capture
                    .ok_or(RobotError::ParsingError)?
                    .as_str()
                    .parse::<i32>()
                    .map_err(|_| RobotError::ParsingError)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let robot = Robot {
            start_position: Complex::new(captured_numbers[0], captured_numbers[1]),
            current_position: Complex::new(captured_numbers[0], captured_numbers[1]),
            velocity: Complex::new(captured_numbers[2], captured_numbers[3]),
        };
        Ok(robot)
    }
}

impl Robot {
    fn position_at_time(&self, time: i32, width: i32, height: i32) -> Complex<i32> {
        let position = self.velocity * time + self.start_position;
        let x = position.re.rem_euclid(width);
        let y = position.im.rem_euclid(height);
        Complex::new(x, y)
    }

    fn move_step(&mut self, width: i32, height: i32) {
        let position = self.velocity + self.current_position;
        let x = position.re.rem_euclid(width);
        let y = position.im.rem_euclid(height);
        self.current_position = Complex::new(x, y);
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn div_ceil(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

fn display_grid(robots: &[Robot], width: i32, height: i32) -> String {
    let mut grid = vec![vec![' '; width as usize]; height as usize];
    for robot in robots {
        grid[robot.current_position.im as usize][robot.current_position.re as usize] = '#';
    }
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn detect_image(map: &str) -> bool {
    let mut consecutive = 0;
    for char in map.chars() {
        if char == '#' {
            consecutive += 1;
        } else {
            consecutive = 0;
        }
        if consecutive == 5 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
