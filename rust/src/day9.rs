use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u32 as u32_nom};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};
use nom::IResult;
use petgraph::prelude::UnGraphMap;

#[aoc_generator(day9)]
fn parse_routes(input: &str) -> Vec<Route> {
    separated_list1(line_ending, route)(input).unwrap().1
}

#[aoc(day9, part1)]
pub fn solve_part1(routes: &[Route]) -> usize {
    let graph: UnGraphMap<&str, u32> = UnGraphMap::from_edges(routes.iter().map(|route| {
        (
            route.from.as_str().clone(),
            route.to.as_str().clone(),
            route.distance,
        )
    }));

    graph
        .nodes()
        .permutations(8)
        .fold(usize::MAX, |min: usize, scenario: Vec<&str>| {
            min.min(
                scenario
                    .iter()
                    .tuple_windows()
                    .map(|(&a, &b)| graph.edge_weight(a, b).unwrap())
                    .sum::<u32>() as usize,
            )
        })
}

#[aoc(day9, part2)]
pub fn solve_part2(routes: &[Route]) -> usize {
    let graph: UnGraphMap<&str, u32> = UnGraphMap::from_edges(routes.iter().map(|route| {
        (
            route.from.as_str().clone(),
            route.to.as_str().clone(),
            route.distance,
        )
    }));

    graph
        .nodes()
        .permutations(8)
        .fold(usize::MIN, |acc: usize, scenario: Vec<&str>| {
            acc.max(
                scenario
                    .iter()
                    .tuple_windows()
                    .map(|(&a, &b)| graph.edge_weight(a, b).unwrap())
                    .sum::<u32>() as usize,
            )
        })
}

pub struct Route {
    from: String,
    to: String,
    distance: u32,
}
fn route(input: &str) -> IResult<&str, Route> {
    map(
        tuple((city_to_city, preceded(tag(" = "), u32_nom))),
        |((from, to), distance)| Route { from, to, distance },
    )(input)
}
fn city_to_city(input: &str) -> IResult<&str, (String, String)> {
    map(
        separated_pair(alpha1, tag(" to "), alpha1),
        |(from, to): (&str, &str)| (from.to_string(), to.to_string()),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::day9::{city_to_city, route};

    #[test]
    fn test_distance() {
        let (tail, route) = route("London to Dublin = 464").unwrap();

        assert_eq!("", tail);
        assert_eq!("London".to_string(), route.from);
        assert_eq!("Dublin".to_string(), route.to);
        assert_eq!(464, route.distance);
    }
    #[test]
    fn test_city_to_city() {
        let (tail, (from, to)) = city_to_city("London to Dublin").unwrap();

        assert_eq!("", tail);
        assert_eq!("London".to_string(), from);
        assert_eq!("Dublin".to_string(), to);
    }
}
