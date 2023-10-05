use aoc_runner_derive::aoc;
use std::ops::Div;
use std::str::FromStr;
use std::usize;

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> usize {
    let min = usize::from_str(input).unwrap().div(10);
    let mut houses = [0; 1_000_000];
    let max = houses.len() - 1;
    (1..)
        .find(|&elf| {
            (elf..=max)
                .step_by(elf)
                .filter(|&h| h < max)
                .for_each(|house| houses[house] += elf);
            houses[elf] >= min
        })
        .unwrap()
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> usize {
    let min = usize::from_str(input).unwrap().div(11);
    let mut houses = [0; 1_000_000];
    let max = houses.len() - 1;
    (1..)
        .find(|&elf| {
            (1..=50)
                .filter_map(|step| match step * elf {
                    house if house < max => Some(house),
                    _ => None,
                })
                .for_each(|house| houses[house] += elf);
            houses[elf] >= min
        })
        .unwrap()
}
