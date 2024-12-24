use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use std::{collections::HashSet, str::FromStr};
advent_of_code::solution!(23);

#[derive(Debug, PartialEq, Eq, Default)]
struct ComputerId {
    id: (char, char),
}

impl From<ComputerId> for NodeIndex {
    fn from(val: ComputerId) -> Self {
        let index = (val.id.0 as u32) << 8 | (val.id.1 as u32);
        NodeIndex::new(index as usize)
    }
}

impl From<NodeIndex> for ComputerId {
    fn from(val: NodeIndex) -> Self {
        let index = val.index();
        let id = (
            ((index >> 8) & 0xFF) as u8 as char,
            (index & 0xFF) as u8 as char,
        );
        ComputerId { id }
    }
}

#[derive(Debug)]
struct ComputerIdError;

impl FromStr for ComputerId {
    type Err = ComputerIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ComputerIdError);
        }
        let mut s_chars = s.chars();
        let id = (
            s_chars.next().ok_or(ComputerIdError)?,
            s_chars.next().ok_or(ComputerIdError)?,
        );
        Ok(ComputerId { id })
    }
}

fn find_games(graph: &UnGraph<ComputerId, ()>) -> HashSet<[NodeIndex; 3]> {
    let mut result = HashSet::new();
    for node in graph.node_indices() {
        let neighbors = graph.neighbors(node).collect_vec();
        for neighbor in neighbors.iter() {
            for second_neighbor in graph.neighbors(*neighbor) {
                if second_neighbor != node && neighbors.contains(&second_neighbor) {
                    let mut game = [node, *neighbor, second_neighbor];
                    game.sort_unstable();
                    result.insert(game);
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let connections = input
        .lines()
        .map(|line| line.trim().split_once('-').unwrap())
        .map(|(a, b)| {
            (
                a.parse::<ComputerId>().unwrap(),
                b.parse::<ComputerId>().unwrap(),
            )
        });

    let network = UnGraph::<ComputerId, ()>::from_edges(connections);

    let games = find_games(&network);
    let games_count = games
        .iter()
        .filter(|&game| game.iter().any(|node| ComputerId::from(*node).id.0 == 't'))
        .count();

    Some(games_count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
