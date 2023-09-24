use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter(at_least_3_vowels)
        .filter(duplicate_letters)
        .filter(not_restricted)
        .count()
}

fn at_least_3_vowels(text: &&str) -> bool {
    text.chars()
        .filter(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .count()
        >= 3
}

fn duplicate_letters(text: &&str) -> bool {
    text.chars().tuple_windows().find(|(a, b)| a == b).is_some()
}

fn not_restricted(text: &&str) -> bool {
    text.chars()
        .tuple_windows()
        .find(|&pair| match pair {
            ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => true,
            _ => false,
        })
        .is_none()
}

#[cfg(test)]
mod test {
    use crate::day5::{at_least_3_vowels, duplicate_letters, not_restricted, solve_part1};

    #[test]
    fn test_vowels() {
        assert_eq!(true, at_least_3_vowels(&"aei"));
        assert_eq!(true, at_least_3_vowels(&"xazegov"));
        assert_eq!(true, at_least_3_vowels(&"aeiouaeiouaeiou"));
    }
    #[test]
    fn test_duplicates() {
        assert_eq!(true, duplicate_letters(&"xx"));
        assert_eq!(true, duplicate_letters(&"abcdde"));
        assert_eq!(true, duplicate_letters(&"aabbccdd"));
    }
    #[test]
    fn test_restrictions() {
        assert_eq!(false, not_restricted(&"ab"));
        assert_eq!(false, not_restricted(&"cd"));
        assert_eq!(false, not_restricted(&"pq"));
        assert_eq!(false, not_restricted(&"xy"));
    }
    #[test]
    fn part1() {
        let input = [
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ]
        .join("\n");

        assert_eq!(2, solve_part1(input.as_str()));
    }
}
