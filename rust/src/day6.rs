use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Product};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char as char_nom, line_ending, multispace1, u32 as u32_nom};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fmt::{Display, Formatter};
use std::ops::RangeInclusive;

#[aoc_generator(day6)]
fn parse_instructions(input: &str) -> Vec<(Action, Window)> {
    instructions(input).unwrap().1
}

type Point = (usize, usize);

#[aoc(day6, part1, Array)]
pub fn solve_part1(instructions: &[(Action, Window)]) -> usize {
    let mut lights = [false; 1_000_000];
    for (action, window) in instructions {
        for (row, column) in window.range() {
            let idx = row * 1000 + column;
            lights[idx] = match action {
                Action::Toggle => !lights[idx],
                Action::On => true,
                Action::Off => false,
            };
        }
    }
    lights.iter().filter(|light| **light).count()
}

#[aoc(day6, part2)]
pub fn solve_part2(instructions: &[(Action, Window)]) -> u32 {
    let mut lights = [0u32; 1_000_000];
    for (action, window) in instructions {
        for (row, col) in window.range() {
            let idx = row * 1000 + col;
            lights[idx] = match action {
                Action::On => lights[idx] + 1,
                Action::Off => lights[idx].saturating_sub(1),
                Action::Toggle => lights[idx] + 2,
            }
        }
    }
    lights.iter().sum::<u32>()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Window {
    from: Point,
    to: Point,
}
impl Display for Window {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} through {},{}",
            self.from.0, self.from.1, self.to.0, self.to.1
        )
    }
}

impl Window {
    fn range(&self) -> Product<RangeInclusive<usize>, RangeInclusive<usize>> {
        (self.from.0..=(self.to.0)).cartesian_product(self.from.1..=(self.to.1))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Toggle,
    On,
    Off,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Toggle => write!(f, "toggle"),
            Action::On => write!(f, "turn on"),
            Action::Off => write!(f, "turn off"),
        }
    }
}

fn instructions(input: &str) -> IResult<&str, Vec<(Action, Window)>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, (Action, Window)> {
    separated_pair(action, multispace1, window)(input)
}
fn action(input: &str) -> IResult<&str, Action> {
    alt((toggle, turn_on, turn_off))(input)
}

fn toggle(input: &str) -> IResult<&str, Action> {
    map(tag("toggle"), |_| Action::Toggle)(input)
}
fn turn_on(input: &str) -> IResult<&str, Action> {
    map(tag("turn on"), |_| Action::On)(input)
}
fn turn_off(input: &str) -> IResult<&str, Action> {
    map(tag("turn off"), |_| Action::Off)(input)
}

fn point(input: &str) -> IResult<&str, (usize, usize)> {
    map(separated_pair(u32_nom, char_nom(','), u32_nom), |(r, c)| {
        (r as usize, c as usize)
    })(input)
}
fn window(input: &str) -> IResult<&str, Window> {
    map(
        separated_pair(point, tag(" through "), point),
        |(from, to)| Window { from, to },
    )(input)
}

#[cfg(test)]
mod test {
    use crate::day6::Action::On;
    use crate::day6::{instruction, solve_part1, solve_part2, Window};

    #[test]
    fn parse_instruction() {
        assert_eq!(
            Ok((
                "",
                (
                    On,
                    Window {
                        from: (0, 0),
                        to: (999, 999)
                    }
                )
            )),
            instruction("turn on 0,0 through 999,999")
        );
    }
    #[test]
    fn part1_examples() {
        let instructions = [
            instruction("turn on 0,0 through 999,999").unwrap().1,
            instruction("toggle 0,0 through 999,0").unwrap().1,
            instruction("turn off 499,499 through 500,500").unwrap().1,
        ];
        let expected = 1_000_000 - 1000 - 4; // All on, minus the first row, minus middle 4

        assert_eq!(expected, solve_part1(&instructions))
    }

    #[test]
    fn test_part2() {
        let instructions = [
            instruction("turn on 0,0 through 0,0").unwrap().1,
            instruction("toggle 0,0 through 999,999").unwrap().1,
        ];

        assert_eq!(2000001, solve_part2(&instructions));
    }
}
