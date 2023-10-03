use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;

#[aoc(day19, part1)]
fn solve_part1(input: &str) -> usize {
    let (replacements, subject) = parse_input(input.trim()).unwrap().1;
    replacements
        .iter()
        .flat_map(|repl| repl.matches(subject))
        .unique()
        .count()
}

#[aoc(day19, part2)]
fn solve_part2(input: &str) -> usize {
    let (replacements, destination) = parse_input(input.trim()).unwrap().1;
    replacements
        .iter()
        .flat_map(|repl| repl.matches(destination))
        .unique()
        .count()
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Replacement>, &str)> {
    separated_pair(replacements, many1(line_ending), alpha1)(input)
}
fn replacements(input: &str) -> IResult<&str, Vec<Replacement>> {
    separated_list1(line_ending, replacement)(input)
}
fn replacement(input: &str) -> IResult<&str, Replacement> {
    map(
        separated_pair(alpha1, tag(" => "), alpha1),
        |(token, replacement)| Replacement {
            from: token,
            to: replacement,
        },
    )(input)
}
#[derive(Debug)]
struct Replacement<'a> {
    from: &'a str,
    to: &'a str,
}

impl Replacement<'_> {
    fn matches(&self, subject: &str) -> Vec<String> {
        subject
            .match_indices(self.from)
            .map(|(match_idx, s)| {
                format!(
                    "{}{}{}",
                    &subject[0..match_idx],
                    self.to,
                    &subject[(match_idx + s.len())..]
                )
            })
            .collect_vec()
    }
}

#[cfg(test)]
mod test {
    use crate::day19::{parse_input, solve_part1};

    const EXAMPLE: &str = r"H => HO
H => OH
O => HH

HOH";
    #[test]
    fn parse_example() {
        let (tail, (replacements, subject)) = parse_input(EXAMPLE.trim()).unwrap();
        assert_eq!("", tail);
        assert_eq!("HOH", subject);
        assert_eq!(3, replacements.len());
    }

    #[test]
    fn example() {
        assert_eq!(4, solve_part1(EXAMPLE));
    }
}
