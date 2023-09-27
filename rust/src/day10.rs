use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::iter::once;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    tumble(input.trim(), 40).len()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    tumble(input.trim(), 50).len()
}
fn tumble(input: &str, repetitions: usize) -> String {
    (0..repetitions).fold(input.to_string(), |acc, _| look_and_say(acc))
}

fn look_and_say(input: String) -> String {
    let mut itr = input.chars().peekable();
    let mut stuff = vec![];
    while let Some(current) = itr.next() {
        stuff.push(
            once(current)
                .chain(itr.peeking_take_while(|&next| current == next))
                .collect_vec(),
        );
    }
    stuff
        .iter()
        .map(|repeats| (repeats.len(), *repeats.first().unwrap()))
        .map(|(count, character)| format!("{count}{character}"))
        .join("")
}

#[cfg(test)]
mod test {
    use crate::day10::{look_and_say, tumble};

    #[test]
    fn example() {
        assert_eq!("312211", tumble("1", 5));
    }

    #[test]
    fn look_and_say_examples() {
        let scenarios = [
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
            ("111221", "312211"),
        ];
        for (input, expected) in scenarios {
            assert_eq!(look_and_say(input.to_string()), expected);
        }
    }
}
