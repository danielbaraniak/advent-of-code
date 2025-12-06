use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use ndarray::{array, Array2};

advent_of_code::solution!(21);

fn num_pad() -> Array2<char> {
    array![
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['#', '0', 'A']
    ]
}

fn directional_pad() -> Array2<char> {
    array![['#', '^', 'A'], ['<', 'v', '>']]
}

const DIRECTION_ORDER: [char; 4] = ['<', 'v', '>', '^'];
fn dir_pad_comparator(a: &char, b: &char) -> Ordering {
    let a_pos = DIRECTION_ORDER.iter().position(|c| a == c).unwrap();
    let b_pos = DIRECTION_ORDER.iter().position(|c| b == c).unwrap();
    Ord::cmp(&a_pos, &b_pos)
}

fn move_to_key(
    from_position: (usize, usize),
    to_position: (usize, usize),
    keypad: &Array2<char>,
) -> Vec<char> {
    match (keypad.dim().0, from_position, to_position) {
        (4, (3, from_y), (to_x, 0)) => {
            let mut steps = Vec::new();
            steps.push('^');
            match to_x {
                1 => {
                    steps.push('^');
                }
                0 => {
                    steps.push('^');
                    steps.push('^');
                }
                _ => {}
            }
            if from_y == 2 {
                steps.push('<');
            };
            steps.push('<');
            steps.push('A');
            return steps;
        }
        (4, (from_x, 0), (3, to_y)) => {
            let mut steps = Vec::new();
            steps.push('>');
            if to_y == 2 {
                steps.push('>');
            }
            steps.push('v');
            match from_x {
                1 => {
                    steps.push('v');
                }
                0 => {
                    steps.push('v');
                    steps.push('v');
                }
                _ => {}
            }

            steps.push('A');
            return steps;
        }
        (2, (0, from_y), (1, 0)) => {
            let mut steps = Vec::new();
            steps.push('v');
            steps.push('<');
            if from_y == 2 {
                steps.push('<');
            }
            steps.push('A');
            return steps;
        }
        _ => {}
    }

    let dx = to_position.1 as isize - from_position.1 as isize;
    let dy = to_position.0 as isize - from_position.0 as isize;

    let dx_steps = match dx {
        1.. => vec!['>'; dx as usize],
        ..=-1 => vec!['<'; -dx as usize],
        0 => Vec::new(),
    };

    let dy_steps = match dy {
        1.. => vec!['v'; dy as usize],
        ..=-1 => vec!['^'; -dy as usize],
        0 => Vec::new(),
    };

    let mut steps: Vec<char> = dx_steps;
    steps.extend(dy_steps);
    steps.sort_by(dir_pad_comparator);
    steps.push('A');
    steps
}

fn get_key_position(keypad: &Array2<char>, key: char) -> (usize, usize) {
    keypad
        .indexed_iter()
        .find(|(_position, &ch)| ch == key)
        .unwrap()
        .0
}

fn map_keypad(keypad: &Array2<char>, mut code: String) -> String {
    code.insert(0, 'A');
    code.chars()
        .tuple_windows()
        .flat_map(|(c1, c2)| {
            let p1 = get_key_position(keypad, c1);
            let p2 = get_key_position(keypad, c2);
            move_to_key(p1, p2, keypad)
        })
        .collect()
}

fn map_keypad_2(
    keypad: &Array2<char>,
    code: &mut str,
    iterations: u8,
    cache: &mut HashMap<(u8, char, char), Vec<char>>,
) -> String {
    let result: Vec<char> = code
        .chars()
        .tuple_windows()
        .flat_map(|(c1, c2)| get_steps(keypad, c1, c2, iterations, cache))
        .collect();

    let mut prefix = get_steps_with_prefix(keypad, code.chars().next().unwrap(), iterations, cache);

    prefix.extend(result);

    prefix.iter().collect()
}

fn get_steps(
    keypad: &Array2<char>,
    c1: char,
    c2: char,
    iterations: u8,
    cache: &mut HashMap<(u8, char, char), Vec<char>>,
) -> Vec<char> {
    if let Some(cached_steps) = cache.get(&(iterations, c1, c2)) {
        return cached_steps.clone();
    }
    let p1 = get_key_position(keypad, c1);
    let p2 = get_key_position(keypad, c2);
    let steps = move_to_key(p1, p2, keypad);
    if iterations == 0 {
        return steps;
    }

    let steps_to_end: Vec<char> = steps
        .iter()
        .tuple_windows()
        .flat_map(|(&c1_tmp, &c2_tmp)| get_steps(keypad, c1_tmp, c2_tmp, iterations - 1, cache))
        .collect();

    cache.insert((iterations, c1, c2), steps_to_end.clone());
    steps_to_end
}

fn get_steps_with_prefix(
    keypad: &Array2<char>,
    c: char,
    iterations: u8,
    cache: &mut HashMap<(u8, char, char), Vec<char>>,
) -> Vec<char> {
    if let Some(cached_steps) = cache.get(&(iterations, 'A', c)) {
        return cached_steps.clone();
    }
    let p1 = get_key_position(keypad, 'A');
    let p2 = get_key_position(keypad, c);
    let steps = move_to_key(p1, p2, keypad);
    if iterations == 0 {
        return steps;
    }

    let steps_to_end: Vec<char> = steps
        .iter()
        .tuple_windows()
        .flat_map(|(&c1_tmp, &c2_tmp)| get_steps(keypad, c1_tmp, c2_tmp, iterations - 1, cache))
        .collect();

    if steps_to_end.is_empty() {
        cache.insert((iterations, 'A', c), steps.clone());
        return steps;
    }
    let mut prefix = get_steps_with_prefix(keypad, steps_to_end[0], iterations - 1, cache);

    prefix.extend(steps_to_end);

    cache.insert((iterations, 'A', c), prefix.clone());
    prefix
}

fn parse_input_code(input_code: String) -> u32 {
    input_code.strip_suffix("A").unwrap().parse().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let codes: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let numpad = num_pad();
    let directional_pad = directional_pad();

    let mut directional_codes = Vec::new();

    for code in codes.iter() {
        let code1 = map_keypad(&numpad, code.clone());
        let code2 = map_keypad(&directional_pad, code1);
        let code3 = map_keypad(&directional_pad, code2);
        directional_codes.push(code3);
    }
    dbg!(&directional_codes);
    let total_complexity = codes
        .iter()
        .zip(directional_codes)
        .map(|(input_code, output_code)| {
            parse_input_code(input_code.clone()) * output_code.len() as u32
        })
        .sum();
    Some(total_complexity)
}

pub fn part_two(input: &str) -> Option<u32> {
    let codes: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    let mut directional_codes = Vec::new();
    let numpad = num_pad();
    let directional_pad = directional_pad();

    for code in codes.iter() {
        directional_codes.push(map_keypad(&numpad, code.clone()));
    }

    let mut cache = HashMap::new();
    for code in directional_codes.iter_mut() {
        let new_code = map_keypad_2(&directional_pad, code, 3, &mut cache);
        *code = new_code;
    }

    let total_complexity = codes
        .iter()
        .zip(directional_codes)
        .map(|(input_code, output_code)| {
            parse_input_code(input_code.clone()) * output_code.len() as u32
        })
        .sum();
    Some(total_complexity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
