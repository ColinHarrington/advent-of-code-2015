use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::ops::Range;
use std::str::FromStr;

#[aoc_generator(day17)]
fn parse_containers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str(line).unwrap())
        .collect_vec()
}

#[aoc(day17, part1)]
fn solve_part1(containers: &[u32]) -> usize {
    combo_count(150, containers)
}

#[aoc(day17, part2)]
fn solve_part2(containers: &[u32]) -> usize {
    minimim_combos(150, containers)
}

fn minimim_combos(liters: u32, containers: &[u32]) -> usize {
    let max = 2u32.pow(containers.len() as u32);
    let ones_range = combo_range(liters, containers);
    let combos = (1..max)
        .filter(|v| ones_range.contains(&v.count_ones()))
        .map(|bits| {
            (0..24)
                .filter(|idx| (bits & (1 << idx)) != 0)
                .map(|idx| containers[idx])
                .collect_vec()
        })
        .filter(|combo| combo.iter().sum::<u32>() == liters)
        .map(|combo| combo.len())
        .collect_vec();

    let min = *combos.iter().min().unwrap();
    combos.into_iter().filter(|&size| size == min).count()
}

fn combo_count(liters: u32, containers: &[u32]) -> usize {
    let max = 2u32.pow(containers.len() as u32);
    let ones_range = combo_range(liters, containers);
    (1..max)
        .filter(|v| ones_range.contains(&v.count_ones()))
        .map(|bits| {
            (0..24)
                .filter(|idx| (bits & (1 << idx)) != 0)
                .map(|idx| containers[idx])
                .sum::<u32>()
        })
        .filter(|&sum| sum == liters)
        .count()
}

fn combo_range(liters: u32, containers: &[u32]) -> Range<u32> {
    min_combo(liters, containers)..max_combo(liters, containers)
}
fn max_combo(liters: u32, containers: &[u32]) -> u32 {
    let mut sum = 0;
    containers
        .iter()
        .sorted_by(|a, b| a.cmp(b))
        .map(|container| {
            sum += container;
            sum
        })
        .filter(|&sum| sum <= liters)
        .count() as u32
        + 1
}

fn min_combo(liters: u32, containers: &[u32]) -> u32 {
    let mut sum = 0;
    containers
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .map(|container| {
            sum += container;
            sum
        })
        .filter(|&sum| sum <= liters)
        .count() as u32
        + 1
}

#[cfg(test)]
mod test {
    use crate::day17::{combo_count, minimim_combos};

    #[test]
    fn example_count() {
        let containers = [20, 15, 10, 5, 5];
        assert_eq!(4, combo_count(25, &containers));
    }

    #[test]
    fn example_min_count() {
        let containers = [20, 15, 10, 5, 5];
        assert_eq!(3, minimim_combos(25, &containers));
    }
}
