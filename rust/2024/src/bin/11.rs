use std::collections::HashMap;

advent_of_code::solution!(11);

const ITERATIONS: u32 = 25;

fn count_leaves_at_level(cache: &mut Vec<HashMap<u64, u64>>, n: u64, levels_left: u32) -> u64 {
    if levels_left == 0 {
        return 1;
    }
    if let Some(count) = cache[levels_left as usize].get(&n) {
        return *count;
    }

    if n == 0 {
        let count = count_leaves_at_level(cache, 1, levels_left - 1);
        cache[levels_left as usize].insert(n, count);
        return count;
    }

    let n_digits = n.to_string();

    if n_digits.len() % 2 == 0 {
        let (l_n, r_n) = n_digits.split_at(n_digits.len() / 2);
        let count_l = count_leaves_at_level(cache, l_n.parse().unwrap(), levels_left - 1);
        let count_r = count_leaves_at_level(cache, r_n.parse().unwrap(), levels_left - 1);
        let count = count_l + count_r;
        cache[levels_left as usize].insert(n, count);
        return count;
    }

    let count = count_leaves_at_level(cache, n * 2024, levels_left - 1);
    cache[levels_left as usize].insert(n, count);
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cache: Vec<HashMap<u64, u64>> = vec![HashMap::new(); (ITERATIONS + 1) as usize];

    let count_all = initial_numbers
        .iter()
        .map(|n| count_leaves_at_level(&mut cache, *n, ITERATIONS))
        .sum();

    Some(count_all)
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_numbers: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let iterations = 75;
    let mut cache: Vec<HashMap<u64, u64>> = vec![HashMap::new(); (iterations + 1) as usize];

    let count_all = initial_numbers
        .iter()
        .map(|n| count_leaves_at_level(&mut cache, *n, iterations))
        .sum();

    Some(count_all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
