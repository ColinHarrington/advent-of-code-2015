use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;

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
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        >= 3
}

fn duplicate_letters(text: &&str) -> bool {
    text.chars().tuple_windows().any(|(a, b)| a == b)
}

fn not_restricted(text: &&str) -> bool {
    text.chars()
        .tuple_windows()
        .all(|pair: CharPair| !matches!(pair, ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .filter(paired_twice)
        .filter(paired_triplet)
        .count()
}

type CharPair = (char, char);

fn paired_twice(text: &&str) -> bool {
    let mut pairs: HashMap<CharPair, usize> = HashMap::new();

    text.chars().tuple_windows().enumerate().any(|(idx, pair)| {
        if let Some(existing_idx) = pairs.get(&pair) {
            idx - existing_idx >= 2
        } else {
            pairs.insert(pair, idx);
            false
        }
    })
}
fn paired_triplet(text: &&str) -> bool {
    text.chars()
        .collect_vec()
        .windows(3)
        .any(|w| w.len() == 3 && w[0] == w[2])
}

#[cfg(test)]
mod test {
    use crate::day5::{
        at_least_3_vowels, duplicate_letters, not_restricted, paired_triplet, paired_twice,
        solve_part1, solve_part2,
    };

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

    #[test]
    fn duplicate_twice() {
        assert_eq!(true, paired_twice(&"xyxy"));
        assert_eq!(true, paired_twice(&"aabcdefgaa"));
        assert_eq!(false, paired_twice(&"aaa"));
    }

    #[test]
    fn separated_pair() {
        assert_eq!(true, paired_triplet(&"xyx"));
        assert_eq!(true, paired_triplet(&"abcdefeghi"));
        assert_eq!(true, paired_triplet(&"aaa"));
    }
    #[test]
    fn part2() {
        let input = [
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ]
        .join("\n");

        assert_eq!(2, solve_part2(input.as_str()));
    }
}
