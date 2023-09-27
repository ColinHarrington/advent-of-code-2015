use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, i32 as i32_nom, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;
use petgraph::graphmap::DiGraphMap;

#[aoc_generator(day13)]
pub fn parse_happy_dances(input: &str) -> Vec<HappyDance> {
    happy_dances(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn solve_part1(dances: &[HappyDance]) -> i32 {
    let graph: DiGraphMap<&str, i32> = table_graph(dances);

    graph.nodes().permutations(graph.node_count()).fold(
        i32::MIN,
        |max: i32, scenario: Vec<&str>| {
            max.max(
                scenario
                    .iter()
                    .circular_tuple_windows()
                    .map(|(&a, &b)| {
                        graph.edge_weight(a, b).unwrap() + graph.edge_weight(b, a).unwrap()
                    })
                    .sum(),
            )
        },
    )
}

#[aoc(day13, part2)]
pub fn solve_part2(dances: &[HappyDance]) -> i32 {
    let graph: DiGraphMap<&str, i32> = table_graph_with_self(dances);

    graph.nodes().permutations(graph.node_count()).fold(
        i32::MIN,
        |max: i32, scenario: Vec<&str>| {
            max.max(
                scenario
                    .iter()
                    .circular_tuple_windows()
                    .map(|(&a, &b)| {
                        graph.edge_weight(a, b).unwrap_or(&0)
                            + graph.edge_weight(b, a).unwrap_or(&0)
                    })
                    .sum(),
            )
        },
    )
}

fn table_graph(dances: &[HappyDance]) -> DiGraphMap<&str, i32> {
    DiGraphMap::from_edges(
        dances
            .iter()
            .map(|(from, to, happiness)| (from.as_str(), to.as_str(), *happiness)),
    )
}

fn table_graph_with_self(dances: &[HappyDance]) -> DiGraphMap<&str, i32> {
    let mut graph: DiGraphMap<&str, i32> = DiGraphMap::from_edges(
        dances
            .iter()
            .map(|(from, to, happiness)| (from.as_str(), to.as_str(), *happiness)),
    );
    graph.add_node("Self");
    graph
}

type HappyDance = (String, String, i32);

fn happy_dances(input: &str) -> IResult<&str, Vec<HappyDance>> {
    separated_list1(line_ending, happy_dance)(input)
}
fn happy_dance(input: &str) -> IResult<&str, HappyDance> {
    map(
        tuple((
            terminated(alpha1, tag(" would ")),
            happiness,
            delimited(tag(" by sitting next to "), alpha1, char('.')),
        )),
        |(a, h, b)| (String::from(a), String::from(b), h),
    )(input)
}
fn happiness(input: &str) -> IResult<&str, i32> {
    alt((happiness_gain, happiness_loss))(input)
}

fn happiness_gain(input: &str) -> IResult<&str, i32> {
    delimited(tag("gain "), i32_nom, tag(" happiness units"))(input)
}

fn happiness_loss(input: &str) -> IResult<&str, i32> {
    map(
        delimited(tag("lose "), i32_nom, tag(" happiness units")),
        |value| 0 - value,
    )(input)
}

#[cfg(test)]
mod test {
    use crate::day13::{happy_dance, parse_happy_dances, solve_part1, solve_part2};

    const EXAMPLE: &str = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
    #[test]
    fn example_part1() {
        let dances = parse_happy_dances(EXAMPLE);
        assert_eq!(330, solve_part1(&dances));
    }

    #[test]
    fn example_part2() {
        let dances = parse_happy_dances(EXAMPLE);
        assert_eq!(286, solve_part2(&dances));
    }

    #[test]
    fn parse() {
        let scenarios = [
            (
                ("Alice".to_string(), "Bob".to_string(), 54),
                "Alice would gain 54 happiness units by sitting next to Bob.",
            ),
            (
                ("Alice".to_string(), "Carol".to_string(), -79),
                "Alice would lose 79 happiness units by sitting next to Carol.",
            ),
        ];
        for (expected, line) in scenarios {
            let (tail, dance) = happy_dance(line).unwrap();
            assert_eq!("", tail);
            assert_eq!(expected, dance);
        }
    }
}
