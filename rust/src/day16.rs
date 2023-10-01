use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u32 as u32_nom, u8 as u8_nom};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;
use std::collections::HashMap;

#[aoc_generator(day16)]
fn parse_aunts(input: &str) -> Vec<Sue> {
    sues(input).unwrap().1
}

const CRITERIA: [(&str, u8); 10] = [
    ("children", 3u8),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

#[aoc(day16, part1)]
fn solve_part1(sues: &[Sue]) -> u32 {
    sues.iter()
        .find(|sue| {
            CRITERIA
                .iter()
                .all(|(attribute, value)| sue.retain(attribute, value))
        })
        .unwrap()
        .number
}

#[aoc(day16, part2)]
fn solve_part2(sues: &[Sue]) -> u32 {
    sues.iter()
        .find(|sue| {
            CRITERIA
                .iter()
                .all(|(attribute, value)| sue.retain2(attribute, value))
        })
        .unwrap()
        .number
}

fn sues(input: &str) -> IResult<&str, Vec<Sue>> {
    separated_list1(line_ending, sue)(input)
}

fn sue(input: &str) -> IResult<&str, Sue> {
    map(
        tuple((
            preceded(tag("Sue "), terminated(u32_nom, tag(": "))),
            attributes,
        )),
        |(number, attributes)| Sue { number, attributes },
    )(input)
}

fn attributes(input: &str) -> IResult<&str, HashMap<String, u8>> {
    map(separated_list1(tag(", "), attribute), HashMap::from_iter)(input)
}

fn attribute(input: &str) -> IResult<&str, (String, u8)> {
    map(
        tuple((terminated(attribute_name, tag(": ")), u8_nom)),
        |(name, value)| (name.to_string(), value),
    )(input)
}

fn attribute_name(input: &str) -> IResult<&str, String> {
    map(alpha1, |name: &str| name.to_string())(input)
}

#[derive(Debug)]
struct Sue {
    number: u32,
    attributes: HashMap<String, u8>,
}

impl Sue {
    fn get(&self, attribute: &str) -> Option<&u8> {
        self.attributes.get(attribute)
    }

    fn retain(&self, attribute: &str, value: &u8) -> bool {
        match self.get(attribute) {
            None => true,
            Some(val) => val == value,
        }
    }

    fn retain2(&self, attribute: &str, value: &u8) -> bool {
        match self.get(attribute) {
            None => true,
            Some(val) => match attribute {
                "cats" | "trees" => val > value,
                "pomeranians" | "goldfish" => val < value,
                _ => val == value,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day16::{attribute, attribute_name, attributes, sue};

    #[test]
    fn parse_sues() {
        let (tail, aunt) = sue("Sue 1: goldfish: 6, trees: 9, akitas: 0").unwrap();
        assert_eq!("", tail);
        assert_eq!(6, *aunt.get("goldfish").unwrap())
    }

    #[test]
    fn parse_attribute_name() {
        let (tail, name) = attribute_name("goldfish: 6").unwrap();
        assert_eq!(": 6", tail);
        assert_eq!("goldfish", name.as_str())
    }

    #[test]
    fn parse_attribute() {
        let (tail, (name, val)) = attribute("goldfish: 6").unwrap();
        assert_eq!("", tail);
        assert_eq!("goldfish", name.as_str());
        assert_eq!(6, val)
    }

    #[test]
    fn parse_attributes() {
        let (tail, attributes) = attributes("goldfish: 6, trees: 9, akitas: 0").unwrap();
        assert_eq!("", tail);

        assert_eq!(3, attributes.len());
        assert_eq!(6, *attributes.get("goldfish").unwrap());
        assert_eq!(9, *attributes.get("trees").unwrap());
        assert_eq!(0, *attributes.get("akitas").unwrap());
    }
}
