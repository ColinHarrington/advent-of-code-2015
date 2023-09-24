use aoc_runner_derive::aoc;
use std::collections::BTreeSet;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let start: House = (0, 0);
    let mut visited: BTreeSet<House> = BTreeSet::from([start]);

    input.chars().fold(start, |house, c| {
        let position = next_house(house, c);
        visited.insert(position);
        position
    });
    visited.len()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let start: House = (0, 0);
    let mut visited: BTreeSet<House> = BTreeSet::from([start]);

    let mut santa = start;
    let mut robos = start;

    input.chars().enumerate().for_each(|(i, direction)| {
        if i % 2 == 1 {
            santa = next_house(santa, direction);
            visited.insert(santa);
        } else {
            robos = next_house(robos, direction);
            visited.insert(robos);
        }
    });

    visited.len()
}

type House = (i32, i32);

fn next_house((x, y): House, direction: char) -> House {
    match direction {
        '^' => (x, y + 1),
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        'v' => (x, y - 1),
        _ => panic!("Unexpected direction"),
    }
}
