use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u32 as u32_nom};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;
use std::ops::{Div, Mul, Rem};

#[aoc_generator(day14)]
fn parse_olympic_herd(input: &str) -> Vec<Reindeer> {
    herd(input).unwrap().1
}

#[aoc(day14, part1)]
fn solve_part1(herd: &[Reindeer]) -> u32 {
    herd.iter()
        .map(|reindeer| reindeer.distance_at(2503))
        .max()
        .unwrap()
}

#[aoc(day14, part2)]
fn solve_part2(herd: &[Reindeer]) -> u32 {
    (1..=2503)
        .fold(
            herd.iter().map(|_| 0).collect_vec(),
            |scores: Vec<u32>, seconds| score(scores, seconds, herd),
        )
        .into_iter()
        .max()
        .unwrap()
}

fn score(scores: Vec<u32>, seconds: u32, herd: &[Reindeer]) -> Vec<u32> {
    let distances = herd
        .iter()
        .map(|reindeer| reindeer.distance_at(seconds))
        .collect_vec();
    let max = distances.iter().max().unwrap();
    distances
        .iter()
        .zip(scores.iter())
        .map(|(distance, score)| if distance == max { score + 1 } else { *score })
        .collect_vec()
}
fn herd(input: &str) -> IResult<&str, Vec<Reindeer>> {
    separated_list1(line_ending, reindeer)(input)
}

fn reindeer(input: &str) -> IResult<&str, Reindeer> {
    map(
        tuple((
            alpha1,
            delimited(tag(" can fly "), u32_nom, tag(" km/s for ")),
            duration,
            delimited(tag(", but then must rest for "), duration, tag(".")),
        )),
        |(name, fly_rate, duration, rest)| Reindeer {
            name: name.to_string(),
            fly_rate,
            duration,
            rest,
        },
    )(input)
}

fn duration(input: &str) -> IResult<&str, u32> {
    terminated(u32_nom, tag(" seconds"))(input)
}
#[derive(Debug, Eq, PartialEq)]
pub struct Reindeer {
    name: String,
    fly_rate: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn distance_at(&self, seconds: u32) -> u32 {
        let cycle = self.duration + self.rest;
        let cycles = seconds.div(cycle);
        let remaining = seconds.rem(cycle);
        cycles * self.duration * self.fly_rate + remaining.min(self.duration).mul(self.fly_rate)
        // cycles.mul(self.fly_rate).add(seconds.rem(cycle).min(self.duration).mul(self.fly_rate))
    }
}
#[cfg(test)]
mod test {
    use crate::day14::{reindeer, Reindeer};

    #[test]
    fn parse_reindeer() {
        assert_eq!(
            Reindeer {
                name: "Comet".to_string(),
                fly_rate: 14,
                duration: 10,
                rest: 127
            },
            reindeer("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.")
                .unwrap()
                .1
        );

        assert_eq!(
            Reindeer {
                name: "Dancer".to_string(),
                fly_rate: 16,
                duration: 11,
                rest: 162
            },
            reindeer("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.")
                .unwrap()
                .1
        );
    }

    #[test]
    fn thosandth_second() {
        let comet = Reindeer {
            name: "Comet".to_string(),
            fly_rate: 14,
            duration: 10,
            rest: 127,
        };
        assert_eq!(14, comet.distance_at(1));
        assert_eq!(28, comet.distance_at(2));
        assert_eq!(140, comet.distance_at(10));
        assert_eq!(1120, comet.distance_at(1000));

        let dancer = Reindeer {
            name: "Dancer".to_string(),
            fly_rate: 16,
            duration: 11,
            rest: 162,
        };
        assert_eq!(16, dancer.distance_at(1));
        assert_eq!(32, dancer.distance_at(2));
        assert_eq!(160, dancer.distance_at(10));
        assert_eq!(176, dancer.distance_at(11));
        assert_eq!(176, dancer.distance_at(12));
        assert_eq!(1056, dancer.distance_at(1000));
    }
}
