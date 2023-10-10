use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use rand::seq::SliceRandom;

#[aoc_generator(day19)]
fn parse_rudolph_meds(input: &str) -> RudolphMeds {
    rudolph_meds(input).unwrap().1
}

#[aoc(day19, part1)]
fn solve_part1(input: &RudolphMeds) -> usize {
    input
        .replacements
        .iter()
        .flat_map(|repl| repl.matches(input.medicine.as_str()))
        .unique()
        .count()
}

#[aoc(day19, part2)]
pub fn part2(input: &RudolphMeds) -> usize {
    let mut replacements = input.replacements.clone();

    let mut rng = rand::thread_rng();

    let mut molecule = String::new();
    loop {
        let mut steps = 0;
        let mut mol = input.medicine.clone();
        replacements.shuffle(&mut rng);
        loop {
            for r in &replacements {
                let from = r.to.as_str();
                let to = r.from.as_str();
                let matches = mol.clone().matches(from).count();
                let next = mol.replace(from, to);
                if next == "e" {
                    return steps + matches;
                } else {
                    if next.contains('e') {
                        continue;
                    }
                    mol = next;
                    steps += matches
                }
            }
            if molecule == mol {
                break;
            }
            molecule = mol.clone();
        }
    }
}

fn rudolph_meds(input: &str) -> IResult<&str, RudolphMeds> {
    map(
        separated_pair(replacements, many1(line_ending), alpha1),
        |(replacements, med)| RudolphMeds {
            replacements,
            medicine: med.to_string(),
        },
    )(input)
}

fn replacements(input: &str) -> IResult<&str, Vec<Replacement>> {
    separated_list1(line_ending, replacement)(input)
}

fn replacement(input: &str) -> IResult<&str, Replacement> {
    map(
        separated_pair(alpha1, tag(" => "), alpha1),
        |(token, replacement): (&str, &str)| Replacement {
            from: token.to_string(),
            to: replacement.to_string(),
        },
    )(input)
}

#[derive(Debug)]
pub struct RudolphMeds {
    replacements: Vec<Replacement>,
    medicine: String,
}

#[derive(Debug, Clone)]
struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    fn matches(&self, subject: &str) -> Vec<String> {
        subject
            .match_indices(self.from.as_str())
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
    use crate::day19::{parse_rudolph_meds, rudolph_meds, solve_part1};

    const EXAMPLE: &str = r"H => HO
H => OH
O => HH

HOH";
    #[test]
    fn parse_example() {
        let (tail, red_meds) = rudolph_meds(EXAMPLE.trim()).unwrap();
        assert_eq!("", tail);
        assert_eq!("HOH", red_meds.medicine.as_str());
        assert_eq!(3, red_meds.replacements.len());
    }

    #[test]
    fn example() {
        let rudolph_meds = parse_rudolph_meds(EXAMPLE.trim());
        assert_eq!(4, solve_part1(&rudolph_meds));
    }
}
