use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(22);

const fn mix(secret: u64, n: u64) -> u64 {
    n ^ secret
}

const fn prune(secret: u64) -> u64 {
    secret % 16777216
}

const fn evolve(secret: u64) -> u64 {
    let step1 = prune(mix(secret, secret << 6));
    let step2 = prune(mix(step1, step1 >> 5));
    prune(mix(step2, step2 << 11))
}

pub fn part_one(input: &str) -> Option<u64> {
    let secret_numbers: Vec<u64> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    let total: u64 = secret_numbers
        .into_iter()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = evolve(secret);
            }
            secret
        })
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let initial_numbers: Vec<u64> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    let secret_numbers_per_seller = initial_numbers.into_iter().map(|initial_number| {
        (0..2000).scan(initial_number, |current, _i| {
            *current = evolve(*current);
            Some(*current)
        })
    });

    let sellers_sequences = secret_numbers_per_seller.flat_map(|seller| {
        seller
            .map(|n| (n % 10) as i8)
            .tuple_windows()
            .map(|(a, b)| (b, b - a))
            .tuple_windows::<(_, _, _, _)>()
            .map(|(t1, t2, t3, t4)| ((t1.1, t2.1, t3.1, t4.1), t4.0))
            .unique_by(|(sequence, _)| *sequence)
    });

    let mut sum_per_sequence = HashMap::new();

    for (sequence, price) in sellers_sequences {
        sum_per_sequence
            .entry(sequence)
            .and_modify(|subtotal| *subtotal += price as u32)
            .or_insert(price as u32);
    }

    sum_per_sequence.into_values().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "1
                    2
                    3
                    2024",
        );
        assert_eq!(result, Some(23));
    }
}
