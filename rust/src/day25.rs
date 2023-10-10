use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::u64 as u64_nom;
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::ops::{Add, Div, Mul};

#[aoc_generator(day25)]
fn parse_row_column(input: &str) -> Coordinate {
    coordinate(input).unwrap().1
}

#[aoc(day25, part1)]
fn solve_part1(coordinate: &Coordinate) -> u64 {
    let max = find_iteration(coordinate.row, coordinate.column);

    (1..max).fold(20151125u64, |code, _| {
        code.mul(252533u64).rem_euclid(33554393u64)
    })
}

/// Adjusting for starting at 1 instead of 0-based
///     `(row-1)` and `(column-1)`
/// Adding `(column-1)` to the row to get the start of the diagonal
///
/// ```math
/// n = row - 1 + column - 1
/// n = row + column - 2
///```
/// Sum of ordered numbers
/// ```math
/// (n/2)(start + n) => (n(n+1))/2
/// ```
///
/// Then walk the diagonal by adding the column
/// ```math
/// (n(n+1))/2) + column
/// ```
fn find_iteration(row: u64, column: u64) -> u64 {
    let n = row + column - 2; //

    n.mul(n + 1).div(2).add(column)
    // n.pow(2).add(n).div(2).add(1 + c)
}

fn coordinate(input: &str) -> IResult<&str, Coordinate> {
    map(
        preceded(
            tag("To continue, please consult the code grid in the manual.  Enter the code at "),
            tuple((
                delimited(tag("row "), u64_nom, tag(", ")),
                delimited(tag("column "), u64_nom, tag(".")),
            )),
        ),
        |(row, column)| Coordinate { row, column },
    )(input)
}

struct Coordinate {
    row: u64,
    column: u64,
}

#[cfg(test)]
mod test {
    use crate::day25::find_iteration;

    /// Example:
    ///    | 1   2   3   4   5   6
    /// ---+---+---+---+---+---+---+
    ///  1 |  1   3   6  10  15  21
    ///  2 |  2   5   9  14  20
    ///  3 |  4   8  13  19
    ///  4 |  7  12  18
    ///  5 | 11  17
    ///  6 | 16
    #[test]
    fn iteration_mapping() {
        assert_eq!(1, find_iteration(1, 1));
        assert_eq!(2, find_iteration(2, 1));
        assert_eq!(3, find_iteration(1, 2));
        assert_eq!(21, find_iteration(1, 6));

        assert_eq!(13, find_iteration(3, 3));
        assert_eq!(22, find_iteration(7, 1));
        assert_eq!(52, find_iteration(4, 7));
        assert_eq!(18331560, find_iteration(2981, 3075));
    }
}
