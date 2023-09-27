use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> String {
    Password::from(input).next_valid().to_string()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> String {
    Password::from(input).next_valid().next_valid().to_string()
}
#[derive(Debug, Clone)]
struct Password {
    data: Vec<char>,
}

impl Password {
    fn from(pass: &str) -> Self {
        Password {
            data: pass.chars().collect_vec(),
        }
    }

    fn increment(&mut self) {
        self.data = increment(&self.data);
    }

    fn next_valid(&self) -> Password {
        Password {
            data: self.clone().find(|data| valid_password(data)).unwrap(),
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data.iter().collect::<String>())
    }
}

impl Iterator for Password {
    type Item = Vec<char>;
    fn next(&mut self) -> Option<Vec<char>> {
        self.increment();
        Some(self.data.clone())
    }
}

fn increment(chars: &[char]) -> Vec<char> {
    let (overflow, mut buffer) =
        chars
            .iter()
            .rev()
            .fold((true, vec![]), |(inc, mut buffer), &original| {
                let (overflow, character) = if inc {
                    match increment_char(original) {
                        '{' => (true, 'a'),
                        incremented => (false, incremented),
                    }
                } else {
                    (false, original)
                };
                buffer.push(character);
                (overflow, buffer)
            });
    if overflow {
        buffer.push('a');
    }
    buffer.into_iter().rev().collect_vec()
}

fn increment_char(character: char) -> char {
    match character {
        'i' | 'o' | 'l' => char::from_u32(character as u32 + 2).unwrap(),
        c => char::from_u32(c as u32 + 1).unwrap(),
    }
}

fn valid_password(chars: &[char]) -> bool {
    increasing_trio(chars) && two_non_overlapping_pairs(chars)
}

fn increasing_trio(chars: &[char]) -> bool {
    chars
        .windows(3)
        .filter(|window| window.len() == 3)
        .map(|window| (window[0] as i32, window[1] as i32, window[2] as i32))
        .any(|(a, b, c)| b - a == 1 && c - b == 1)
}

fn two_non_overlapping_pairs(chars: &[char]) -> bool {
    let pairs = chars
        .iter()
        .tuple_windows()
        .enumerate()
        .filter(|(_, (&a, &b))| a == b)
        .map(|(idx, _)| idx)
        .collect_vec();
    match pairs.len() {
        0 | 1 => false,
        2 => pairs.last().unwrap() - pairs.first().unwrap() >= 2,
        _ => true,
    }
}

#[cfg(test)]
mod test {
    use crate::day11::{increasing_trio, increment, two_non_overlapping_pairs};
    use itertools::Itertools;

    #[test]
    fn test_trios() {
        assert_eq!(true, increasing_trio(&"abc".chars().collect_vec()));
        assert_eq!(true, increasing_trio(&"bcd".chars().collect_vec()));
        assert_eq!(true, increasing_trio(&"cde".chars().collect_vec()));
        assert_eq!(true, increasing_trio(&"xyz".chars().collect_vec()));
        assert_eq!(false, increasing_trio(&"abd".chars().collect_vec()));
        assert_eq!(true, increasing_trio(&"hijklmmn".chars().collect_vec()));
    }

    #[test]
    fn test_pairs() {
        let scenarios = [
            (true, "xxasdfyyasdfa"),
            (false, "xxx"),
            (false, ""),
            (false, "xx"),
            (true, "xxxx"),
        ];

        for (expected, password) in scenarios {
            let chars = password.chars().collect_vec();
            assert_eq!(expected, two_non_overlapping_pairs(&chars));
        }
    }

    #[test]
    fn incrementing() {
        let scenarios = vec![
            ("b", "a"),
            ("xy", "xx"),
            ("xz", "xy"),
            ("ya", "xz"),
            ("yb", "ya"),
            ("aa", "z"),
        ]
        .into_iter()
        .map(|(expected, input)| (expected.chars().collect_vec(), input.chars().collect_vec()));

        for (expected, input) in scenarios {
            assert_eq!(expected, increment(&input));
        }
    }
}
