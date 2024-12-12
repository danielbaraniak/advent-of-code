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
    None
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
        assert_eq!(result, None);
    }
}
