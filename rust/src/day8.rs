use aoc_runner_derive::aoc;

const DOUBLE_QUOTE: u8 = 0x22;
const BACKSLASH: u8 = 0x5c;
const X_LOWER: u8 = 0x78;
#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.lines().map(decoded_diff).sum()
}

fn decoded_diff(line: &str) -> usize {
    let bytes = line.as_bytes();

    let mut iter = bytes.iter();
    let mut decoded = vec![];

    while let Some(&byte) = iter.next() {
        decoded.push(match byte {
            BACKSLASH => match *iter.next().unwrap() {
                BACKSLASH => BACKSLASH,
                DOUBLE_QUOTE => DOUBLE_QUOTE,
                X_LOWER => hex_char(*iter.next().unwrap(), *iter.next().unwrap()),
                _ => panic!("unknown Escape"),
            },
            b => b,
        })
    }
    bytes.len() - (decoded.len() - 2usize)
}
fn hex_char(byte1: u8, byte2: u8) -> u8 {
    hex_val(byte1) << 4 | hex_val(byte2)
}
fn hex_val(byte: u8) -> u8 {
    match byte {
        0x30..=0x39 => byte - 0x30,
        0x61..=0x66 => byte - 0x57,
        _ => panic!("non-hex-char"),
    }
}
#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    input.lines().map(encoded_diff).sum()
}

fn encoded_diff(line: &str) -> usize {
    line.as_bytes()
        .iter()
        .fold(2usize, |count, &byte| match byte {
            DOUBLE_QUOTE | BACKSLASH => count + 1,
            _ => count,
        })
}

#[cfg(test)]
mod test {
    use crate::day8::{decoded_diff, encoded_diff};

    #[test]
    fn decoded() {
        assert_eq!(2, decoded_diff("\"\""));
        assert_eq!(2, decoded_diff("\"abc\""));
        assert_eq!(3, decoded_diff("\"aaa\\\"aaa\""));
        assert_eq!(5, decoded_diff("\"\\x27\""));
    }

    #[test]
    fn encoded() {
        assert_eq!(4, encoded_diff("\"\""));
        assert_eq!(4, encoded_diff("\"abc\""));
        assert_eq!(6, encoded_diff("\"aaa\\\"aaa\""));
        assert_eq!(5, encoded_diff("\"\\x27\""));
    }
}
