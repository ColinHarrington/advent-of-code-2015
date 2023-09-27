use aoc_runner_derive::aoc;
use serde_json::Value;
use std::collections::VecDeque;

#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> i64 {
    let mut sum = 0;
    let mut queue: VecDeque<Value> = VecDeque::from([serde_json::from_str(input).unwrap()]);

    while let Some(value) = queue.pop_front() {
        match value {
            Value::Number(n) => {
                sum += n.as_i64().unwrap();
            }
            Value::Object(properties) => properties
                .into_iter()
                .filter(|(_, val)| {
                    matches!(val, Value::Number(_) | Value::Object(_) | Value::Array(_))
                })
                .for_each(|(_, val)| queue.push_back(val)),
            Value::Array(arr) => arr.into_iter().for_each(|element| queue.push_back(element)),
            _ => {}
        }
    }
    sum
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut queue: VecDeque<Value> = VecDeque::from([serde_json::from_str(input).unwrap()]);

    while let Some(value) = queue.pop_front() {
        match value {
            Value::Number(n) => {
                sum += n.as_i64().unwrap();
            }
            Value::Object(properties) if !restricted(&properties) => properties
                .into_iter()
                .filter(|(_, val)| {
                    matches!(val, Value::Number(_) | Value::Object(_) | Value::Array(_))
                })
                .for_each(|(_, val)| queue.push_back(val)),
            Value::Array(arr) => arr.into_iter().for_each(|element| queue.push_back(element)),
            _ => {}
        }
    }
    sum
}

fn restricted(properties: &serde_json::Map<String, Value>) -> bool {
    properties.iter().any(|(_, value)| match value {
        Value::String(s) => s == "red",
        _ => false,
    })
}

#[cfg(test)]
mod test {
    use crate::day12::{solve_part1, solve_part2};

    #[test]
    fn part1_examples() {
        assert_eq!(6, solve_part1("[1,2,3]"));
        assert_eq!(6, solve_part1("{\"a\":2,\"b\":4}"));
        assert_eq!(3, solve_part1("[[[3]]]"));
        assert_eq!(3, solve_part1("{\"a\":{\"b\":4},\"c\":-1}"));
        assert_eq!(0, solve_part1("{\"a\":[-1,1]}"));
        assert_eq!(0, solve_part1("{\"a\":[-1,1]}"));
        assert_eq!(0, solve_part1("[]"));
        assert_eq!(0, solve_part1("{}"));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(6, solve_part2("[1,2,3]"));
        assert_eq!(4, solve_part2("[1,{\"c\":\"red\",\"b\":2},3]"));
        assert_eq!(0, solve_part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"));
        assert_eq!(6, solve_part2("[1,\"red\",5]"));
    }
}
