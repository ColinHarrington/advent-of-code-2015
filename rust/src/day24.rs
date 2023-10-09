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
    min_quantum_entanglement(3, packages)
}

#[aoc(day24, part2)]
fn solve_part2(packages: &[u64]) -> u64 {
    min_quantum_entanglement(4, packages)
}

fn min_quantum_entanglement(partitions: u64, weights: &[u64]) -> u64 {
    let total: u64 = weights.iter().sum();
    assert!(total % partitions == 0);
    let partition_weight: u64 = total / partitions;

    (1..weights.len())
        .find_map(|n| {
            weights
                .iter()
                .combinations(n)
                .filter(|combo| partition_weight == combo.iter().fold(0u64, |acc, &w| w.add(acc)))
                .map(|combo| combo.into_iter().fold(1u64, |acc, w| w.mul(acc)))
                .min()
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::day24::{solve_part1, solve_part2};

    #[test]
    fn example() {
        let packages = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        assert_eq!(99, solve_part1(&packages));
        assert_eq!(44, solve_part2(&packages));
    }
}
