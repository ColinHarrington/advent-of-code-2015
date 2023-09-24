use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u64 {
    (0u64..)
        .into_iter()
        .find(|i| match md5::compute(format!("{input}{i}")).0 {
            [0, 0, third, ..] if third <= 15 => true,
            _ => false,
        })
        .unwrap()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u64 {
    (0u64..)
        .into_iter()
        .find(|i| match md5::compute(format!("{input}{i}")).0 {
            [0, 0, 0, ..] => true,
            _ => false,
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::day4::solve_part1;

    #[test]
    fn part1_example1() {
        assert_eq!(609043, solve_part1("abcdef"));
    }

    #[test]
    fn part1_example2() {
        assert_eq!(1048970, solve_part1("pqrstuv"));
    }
}
