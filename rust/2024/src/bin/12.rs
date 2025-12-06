use itertools::Itertools;
use num_complex::Complex;

advent_of_code::solution!(12);

const DIRECTIONS: [Complex<i32>; 4] = [
    Complex::new(0, 1),
    Complex::new(1, 0),
    Complex::new(0, -1),
    Complex::new(-1, 0),
];

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let is_in_bounds = |coord: &Complex<i32>| {
        0 <= coord.re && coord.re < width && 0 <= coord.im && coord.im < height
    };

    let mut visited = vec![vec![false; width as usize]; height as usize];
    // let edges = vec![vec![false; width as usize + 1]; height as usize + 1];
    let mut total_price = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate() {
            if visited[y][x] {
                continue;
            }
            total_price += calculate_price(
                &map,
                Complex::new(x as i32, y as i32),
                &mut visited,
                is_in_bounds,
            );
        }
    }
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let is_in_bounds = |coord: &Complex<i32>| {
        0 <= coord.re && coord.re < width && 0 <= coord.im && coord.im < height
    };

    let mut visited = vec![vec![false; width as usize]; height as usize];
    // let edges = vec![vec![false; width as usize + 1]; height as usize + 1];
    let mut total_price = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, _) in line.chars().enumerate() {
            if visited[y][x] {
                continue;
            }
            total_price += calculate_price_with_discount(
                &map,
                Complex::new(x as i32, y as i32),
                &mut visited,
                is_in_bounds,
            );
        }
    }
    Some(total_price)
}

fn calculate_price(
    map: &[Vec<char>],
    start: Complex<i32>,
    visited: &mut [Vec<bool>],
    bounds_predicate: impl Fn(&Complex<i32>) -> bool,
) -> u32 {
    let mut area = 1;
    let mut perimeter = 0;
    let mut queue = vec![start];

    visited[start.im as usize][start.re as usize] = true;

    while let Some(current) = queue.pop() {
        let current_type = map[current.im as usize][current.re as usize];
        for direction in DIRECTIONS.iter() {
            let next = current + direction;

            if !bounds_predicate(&next) {
                perimeter += 1;
                continue;
            }

            let next_type = map[next.im as usize][next.re as usize];

            if next_type != current_type {
                perimeter += 1;
                continue;
            }

            if !visited[next.im as usize][next.re as usize] {
                visited[next.im as usize][next.re as usize] = true;
                queue.push(next);
                area += 1;
            }
        }
    }
    area * perimeter
}

fn calculate_price_with_discount(
    map: &[Vec<char>],
    start: Complex<i32>,
    visited: &mut [Vec<bool>],
    bounds_predicate: impl Fn(&Complex<i32>) -> bool,
) -> u32 {
    let mut area = 1;
    let mut queue = vec![start];
    let mut local_visited = vec![vec![false; map[0].len()]; map.len()];

    visited[start.im as usize][start.re as usize] = true;
    local_visited[start.im as usize][start.re as usize] = true;

    let longer_size = map.len().max(map[0].len());

    let mut edges = vec![vec![vec![false; longer_size + 1]; longer_size + 1]; 4];

    while let Some(current) = queue.pop() {
        let current_type = map[current.im as usize][current.re as usize];
        for (i, direction) in DIRECTIONS.iter().enumerate() {
            let next = current + direction;

            if !bounds_predicate(&next) {
                if i % 2 == 0 {
                    edges[i][current.im as usize][current.re as usize] = true;
                } else {
                    edges[i][current.re as usize][next.im as usize] = true;
                }
                continue;
            }

            let next_type = map[next.im as usize][next.re as usize];

            if next_type != current_type {
                if i % 2 == 0 {
                    edges[i][current.im as usize][current.re as usize] = true;
                } else {
                    edges[i][current.re as usize][next.im as usize] = true;
                }
                continue;
            }

            if !visited[next.im as usize][next.re as usize] {
                visited[next.im as usize][next.re as usize] = true;
                local_visited[next.im as usize][next.re as usize] = true;
                queue.push(next);
                area += 1;
            }
        }
    }
    let sides = count_sides(edges);
    area * sides
}

fn count_sides(edges: Vec<Vec<Vec<bool>>>) -> u32 {
    let mut sides = 0;
    for direction in edges.iter() {
        for row in direction.iter() {
            sides += row
                .iter()
                .tuple_windows()
                .map(|(a, b)| if a != b { 1 } else { 0 })
                .sum::<u32>()
                .div_ceil(2);
        }
    }

    sides
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
