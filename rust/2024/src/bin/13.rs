use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let games = parse_input(input);
    let mut total = 0;
    for game in games {
        if let Some(minimum) = find_minimum(game) {
            total += minimum;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut games: Vec<Game> = parse_input(input);
    for game in &mut games {
        game.prize.0 += 10000000000000;
        game.prize.1 += 10000000000000;
    }
    let mut total = 0;
    for game in games {
        if let Some(minimum) = find_minimum(game) {
            total += minimum;
        }
    }
    Some(total)
}

const PATTERN: &str =
    r"Button A: X\+(\d+), Y\+(\d+)\s+Button B: X\+(\d+), Y\+(\d+)\s+Prize: X=(\d+), Y=(\d+)";

lazy_static! {
    static ref game_regex: Regex = Regex::new(PATTERN).unwrap();
}
#[derive(Debug)]
struct Game {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

#[derive(Debug)]
enum GameError {
    ParseGameError,
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captured_numbers = game_regex
            .captures(s)
            .ok_or(GameError::ParseGameError)?
            .iter()
            .skip(1)
            .map(|capture| {
                capture
                    .ok_or(GameError::ParseGameError)?
                    .as_str()
                    .parse::<i64>()
                    .map_err(|_| GameError::ParseGameError)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let game = Game {
            a: (captured_numbers[0], captured_numbers[1]),
            b: (captured_numbers[2], captured_numbers[3]),
            prize: (captured_numbers[4], captured_numbers[5]),
        };
        Ok(game)
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split("\n\n")
        .map(|game_string| Game::from_str(game_string).unwrap())
        .collect()
}

fn find_minimum(game: Game) -> Option<u64> {
    let b_nominator = game.prize.1 * game.a.0 - game.prize.0 * game.a.1;
    let b_denominator = game.a.0 * game.b.1 - game.b.0 * game.a.1;
    let b_result = if b_nominator % b_denominator == 0 {
        b_nominator / b_denominator
    } else {
        return None;
    };

    let a_result = if (game.prize.0 - game.b.0 * b_result) % game.a.0 == 0 {
        (game.prize.0 - game.b.0 * b_result) / game.a.0
    } else {
        return None;
    };

    Some((a_result * 3 + b_result) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
