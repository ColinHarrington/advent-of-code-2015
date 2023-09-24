use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("Unexpected input"),
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut sum = 0;

    input
        .chars()
        .enumerate()
        .find_map(|(i, c)| {
            sum += match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unexpected input"),
            };
            match sum {
                level if level < 0 => Some(i + 1),
                _ => None,
            }
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::day1::{solve_part1, solve_part2};

    /// For example:
    ///  - `(())` and `()()` both result in floor `0`.
    ///  - `(((` and `(()(()(` both result in floor `3`.
    ///  - `))(((((` also results in floor `3`.
    ///  - `())` and `))(` both result in floor `-1` (the first basement level).
    ///  - `)))` and `)())())` both result in floor `-3`.
    #[test]
    fn part1_examples() {
        assert_eq!(0, solve_part1("(())"));
        assert_eq!(0, solve_part1("()()"));
        assert_eq!(3, solve_part1("((("));
        assert_eq!(3, solve_part1("(()(()("));
        assert_eq!(3, solve_part1("(()(()("));
        assert_eq!(-1, solve_part1("())"));
        assert_eq!(-1, solve_part1("))("));
        assert_eq!(-3, solve_part1(")))"));
        assert_eq!(-3, solve_part1(")())())"));
    }

    /// For example:
    /// - `)` causes him to enter the basement at character position `1`.
    /// - `()())` causes him to enter the basement at character position `5`.
    #[test]
    fn part2_examples() {
        assert_eq!(1, solve_part2(")"));
        assert_eq!(5, solve_part2("()())"));
    }
}
