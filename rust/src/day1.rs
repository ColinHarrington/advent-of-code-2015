use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => panic!("Unexpected input")
    })
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut sum = 0;

    input.chars()
        .enumerate()
        .find_map(|(i, c)| {
            sum += match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unexpected input")
            };
            match sum {
                level if level < 0 => Some(i + 1),
                _ => None
            }
        }).unwrap() as u32
}
