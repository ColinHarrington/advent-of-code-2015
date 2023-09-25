use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, u16 as u16_nom};
use nom::combinator::{map, verify};
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::collections::HashMap;

#[aoc_generator(day7)]
fn parse_instructions(input: &str) -> Vec<Assignment> {
    let (tail, defs) = assignments(input).unwrap();
    assert_eq!("", tail);
    defs
}

#[aoc(day7, part1)]
pub fn solve_part1(assignments: &[Assignment]) -> u16 {
    let computer = Computer {
        definitions: HashMap::from_iter(
            assignments
                .iter()
                .map(|(expression, assignment)| (assignment.clone(), expression.clone())),
        ),
    };
    let mut registers: HashMap<String, u16> = HashMap::new();
    let a = "a".to_string();
    computer.eval(&a, &mut registers)
}
#[aoc(day7, part2)]
pub fn solve_part2(assignments: &[Assignment]) -> u16 {
    let computer = Computer {
        definitions: HashMap::from_iter(
            assignments
                .iter()
                .map(|(expression, assignment)| (assignment.clone(), expression.clone())),
        ),
    };
    let mut registers: HashMap<String, u16> = HashMap::new();
    let a = "a".to_string();
    let b_value = computer.eval(&a, &mut registers);
    registers.clear();
    let b = "b".to_string();
    let computer2 = Computer {
        definitions: HashMap::from_iter(assignments.iter().map(|(expression, assignment)| {
            if *assignment == b {
                (assignment.clone(), Expression::Value(b_value))
            } else {
                (assignment.clone(), expression.clone())
            }
        })),
    };
    computer2.eval(&a, &mut registers)
}

#[derive(Debug)]
struct Computer {
    definitions: HashMap<String, Expression>,
}

impl Computer {
    fn eval(&self, register: &String, registers: &mut HashMap<String, u16>) -> u16 {
        if let Some(&value) = registers.get(register) {
            value
        } else {
            let expression = self.definitions.get(register).unwrap();
            let value = match expression {
                Expression::Value(val) => *val,
                Expression::Not(reg) => !self.eval(reg, registers),
                Expression::And(left, right) => {
                    self.eval(left, registers) & self.eval(right, registers)
                }
                Expression::ValueAnd(val, reg) => val & self.eval(reg, registers),
                Expression::Or(left, right) => {
                    self.eval(left, registers) | self.eval(right, registers)
                }
                Expression::Lshift(reg, val) => self.eval(reg, registers) << *val,
                Expression::Rshift(reg, val) => self.eval(reg, registers) >> *val,
                Expression::Register(reg) => self.eval(reg, registers),
            };
            registers.insert(register.clone(), value);
            value
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expression {
    Value(u16),
    Not(String),
    And(String, String),
    ValueAnd(u16, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Register(String),
}

pub type Assignment = (Expression, String);

#[derive(Debug, Eq, PartialEq, Hash)]
struct Register {
    name: String,
}

fn assignments(input: &str) -> IResult<&str, Vec<Assignment>> {
    separated_list1(line_ending, assignment)(input)
}

fn assignment(input: &str) -> IResult<&str, Assignment> {
    separated_pair(expression, tag(" -> "), register)(input)
}

fn register(input: &str) -> IResult<&str, String> {
    map(
        verify(alpha1, |s: &str| s.chars().all(|c| c.is_lowercase())),
        |name: &str| name.to_string(),
    )(input)
}

fn expression(input: &str) -> IResult<&str, Expression> {
    alt((
        value_and_expression,
        value_expression,
        not_expression,
        and_expression,
        or_expression,
        leftshift_expression,
        rightshift_expression,
        register_expression,
    ))(input)
}

fn and_expression(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(register, tag(" AND "), register),
        |(left, right)| Expression::And(left, right),
    )(input)
}
fn value_and_expression(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(u16_nom, tag(" AND "), register),
        |(value, reg)| Expression::ValueAnd(value, reg),
    )(input)
}

fn or_expression(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(register, tag(" OR "), register),
        |(left, right)| Expression::Or(left, right),
    )(input)
}

fn value_expression(input: &str) -> IResult<&str, Expression> {
    map(u16_nom, Expression::Value)(input)
}

fn not_expression(input: &str) -> IResult<&str, Expression> {
    map(preceded(tag("NOT "), register), Expression::Not)(input)
}

fn leftshift_expression(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(register, tag(" LSHIFT "), u16_nom),
        |(left, val)| Expression::Lshift(left, val),
    )(input)
}

fn rightshift_expression(input: &str) -> IResult<&str, Expression> {
    map(
        separated_pair(register, tag(" RSHIFT "), u16_nom),
        |(left, val)| Expression::Rshift(left, val),
    )(input)
}
fn register_expression(input: &str) -> IResult<&str, Expression> {
    map(register, Expression::Register)(input)
}

#[cfg(test)]
mod test {
    use crate::day7::{expression, Expression};

    #[test]
    fn parse_expressions() {
        assert_eq!(
            Expression::Not("e".to_string()),
            expression("NOT e").unwrap().1
        );
        assert_eq!(
            Ok(("", Expression::ValueAnd(1, "cx".to_string()))),
            expression("1 AND cx")
        );
    }
}
