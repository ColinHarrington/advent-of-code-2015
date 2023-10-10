use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::character::complete::{line_ending, u64 as u64_nom};
use nom::multi::separated_list1;
use nom::IResult;
use std::ops::{Add, Mul};

#[aoc_generator(day24)]
fn parse_packages(input: &str) -> Vec<u64> {
    packages(input).unwrap().1
}

fn packages(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(line_ending, u64_nom)(input)
}

#[aoc(day24, part1)]
fn solve_part1(packages: &[u64]) -> u64 {
    min_quantum_entanglement(3, packages).unwrap()
}

#[aoc(day24, part2)]
fn solve_part2(packages: &[u64]) -> u64 {
    min_quantum_entanglement(4, packages).unwrap()
}

fn min_quantum_entanglement(partitions: u64, weights: &[u64]) -> Option<u64> {
    let total: u64 = weights.iter().sum();
    assert!(total % partitions == 0);
    let partition_weight: u64 = total / partitions;

    (1..weights.len()).find_map(|n| {
        weights
            .iter()
            .combinations(n)
            .filter(|combo| partition_weight == combo.iter().fold(0u64, |acc, &w| w.add(acc)))
            .map(|combo| (combo.iter().fold(1u64, |acc, w| w.mul(acc)), combo))
            .sorted_by_key(|(qe, _)| *qe)
            .map(|(qe, combo)| (qe, remaining(&combo, weights)))
            .find_map(
                |(qe, remaining)| match can_partition(&remaining, partitions - 1) {
                    true => Some(qe),
                    false => None,
                },
            )
    })
}

fn can_partition(weights: &Vec<u64>, partitions: u64) -> bool {
    let total: u64 = weights.iter().sum();
    total % partitions == 0
        && backtrack(
            weights,
            0,
            total / partitions,
            0,
            &mut vec![false; weights.len()],
        )
}
fn backtrack(
    weights: &[u64],
    current: u64,
    partition_weight: u64,
    idx: usize,
    excluded: &mut Vec<bool>,
) -> bool {
    if excluded.iter().all(|&b| b) {
        true
    } else {
        for other_idx in idx..weights.len() {
            if excluded[other_idx] || current + weights[other_idx] > partition_weight {
                continue;
            }
            let next = (current + weights[other_idx]) % partition_weight;
            excluded[other_idx] = true;
            if backtrack(
                weights,
                next,
                partition_weight,
                if next == 0 { 0 } else { other_idx + 1 },
                excluded,
            ) {
                return true;
            }
            excluded[other_idx] = false;
        }
        false
    }
}

fn remaining(combo: &Vec<&u64>, packages: &[u64]) -> Vec<u64> {
    let mut remaining = packages.to_vec();
    for &item in combo {
        remaining.remove(remaining.iter().find_position(|&p| p == item).unwrap().0);
    }
    remaining
}

#[cfg(test)]
mod test {
    use crate::day24::{can_partition, solve_part1, solve_part2};

    #[test]
    fn example() {
        let packages = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(99, solve_part1(&packages));
        assert_eq!(44, solve_part2(&packages));
    }

    #[test]
    fn test_can_partition() {
        let weights = [
            2u64, 3, 5, 7, 13, 17, 19, 23, 29, 31, 37, 41, 43, 53, 59, 61, 67, 71, 73, 83, 89, 97,
            101,
        ];
        assert_eq!(true, can_partition(&weights.to_vec(), 2))
    }
}
