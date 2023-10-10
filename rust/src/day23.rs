use aoc_runner_derive::{aoc, aoc_generator};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, i32 as i32_nom, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::HashMap;

#[aoc_generator(day23)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    instructions(input).unwrap().1
}
#[aoc(day23, part1)]
fn solve_part1(instructions: &[Instruction]) -> u32 {
    compute(instructions, [('a', 0), ('b', 0)])
}

#[aoc(day23, part2)]
fn solve_part2(instructions: &[Instruction]) -> u32 {
    compute(instructions, [('a', 1), ('b', 0)])
}

fn compute(instructions: &[Instruction], initial_registers: [(char, u32); 2]) -> u32 {
    let mut registers: HashMap<char, u32> = HashMap::from(initial_registers);
    let mut ip: i32 = 0;

    while let Some(instruction) = instructions.get(ip as usize) {
        ip += match instruction {
            Instruction::Increment(reg) => {
                registers.insert(*reg, registers.get(reg).unwrap() + 1);
                1
            }
            Instruction::Triple(reg) => {
                registers.insert(*reg, registers.get(reg).unwrap() * 3);
                1
            }
            Instruction::Half(reg) => {
                registers.insert(*reg, registers.get(reg).unwrap() / 2);
                1
            }
            Instruction::JumpIfOne(reg, offset) => match registers.get(reg).unwrap() {
                1 => *offset,
                _ => 1,
            },
            Instruction::JumpIfEven(reg, offset) => match registers.get(reg).unwrap() % 2 {
                0 => *offset,
                _ => 1,
            },
            Instruction::Jump(offset) => *offset,
        };
    }
    *registers.get(&'b').unwrap()
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((increment, triple, half, jump, jump_if_even, jump_if_one))(input)
}
fn increment(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("inc "), anychar), Instruction::Increment)(input)
}
fn triple(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("tpl "), anychar), Instruction::Triple)(input)
}
fn half(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("hlf "), anychar), Instruction::Half)(input)
}

fn jump(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("jmp "), i32_nom), Instruction::Jump)(input)
}

fn jump_if_even(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("jie "), tuple((anychar, preceded(tag(", "), i32_nom)))),
        |(register, offset)| Instruction::JumpIfEven(register, offset),
    )(input)
}

fn jump_if_one(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("jio "), tuple((anychar, preceded(tag(", "), i32_nom)))),
        |(register, offset)| Instruction::JumpIfOne(register, offset),
    )(input)
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Increment(char),
    Triple(char),
    Half(char),
    Jump(i32),
    JumpIfEven(char, i32),
    JumpIfOne(char, i32),
}

#[cfg(test)]
mod test {
    use crate::day23::Instruction::{Half, Increment, Jump, JumpIfEven, JumpIfOne, Triple};
    use crate::day23::{half, increment, instructions, jump, jump_if_even, jump_if_one, triple};

    const EXAMPLE: &str = r"inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn parse_instructions() {
        let instructions = instructions(EXAMPLE).unwrap().1;
        assert_eq!(4, instructions.len())
    }
    #[test]
    fn parse_value_instructions() {
        assert_eq!(Half('r'), half("hlf r").unwrap().1);
        assert_eq!(Triple('r'), triple("tpl r").unwrap().1);
        assert_eq!(Increment('r'), increment("inc r").unwrap().1);
    }

    #[test]
    fn parse_jump_instruction() {
        assert_eq!(Jump(1), jump("jmp +1").unwrap().1);
        assert_eq!(Jump(0), jump("jmp +0").unwrap().1);
        assert_eq!(Jump(-7), jump("jmp -7").unwrap().1);
    }

    #[test]
    fn parse_jump_if_even_instruction() {
        assert_eq!(JumpIfEven('r', 9), jump_if_even("jie r, +9").unwrap().1);
        assert_eq!(JumpIfEven('r', 0), jump_if_even("jie r, +0").unwrap().1);
        assert_eq!(JumpIfEven('r', -7), jump_if_even("jie r, -7").unwrap().1);
    }

    #[test]
    fn parse_jump_if_odd_instruction() {
        assert_eq!(JumpIfOne('r', 9), jump_if_one("jio r, +9").unwrap().1);
        assert_eq!(JumpIfOne('r', 0), jump_if_one("jio r, +0").unwrap().1);
        assert_eq!(JumpIfOne('r', -7), jump_if_one("jio r, -7").unwrap().1);
    }
}
