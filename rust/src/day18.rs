use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    grid_steps(input, 100, false)
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> usize {
    grid_steps(input, 100, true)
}

fn grid_steps(input: &str, steps: usize, conway: bool) -> usize {
    let mut grid = Grid::from(input, conway);
    for _ in 1..=steps {
        grid.step();
    }
    grid.data.iter().filter(|(_, &on)| on).count()
}

type Point = (i32, i32);
#[derive(Debug)]
pub struct Grid {
    height: i32,
    width: i32,
    data: HashMap<Point, bool>,
    conway: bool,
}

impl Grid {
    fn from(input: &str, conway: bool) -> Self {
        let mut data: HashMap<Point, bool> =
            HashMap::from_iter(input.lines().enumerate().flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(column, c)| ((row as i32, column as i32), c))
                    .map(|(p, c)| (p, matches!(c, '#')))
                    .collect_vec()
            }));
        let width = data.iter().filter(|((row, _), _)| *row == 0).count() as i32;
        let height = data.iter().map(|((row, _), _)| row).unique().count() as i32;

        if conway {
            let max_width = width - 1;
            let max_height = height - 1;
            data.insert((0, 0), true);
            data.insert((0, max_width), true);
            data.insert((max_height, 0), true);
            data.insert((max_height, max_width), true);
        }

        Self {
            data,
            height,
            width,
            conway,
        }
    }

    fn neighbors(&self, (row, column): &Point) -> Vec<Point> {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(dr, dc)| (row.add(dr), column.add(dc)))
        .filter(|&point| self.contains(point))
        .collect_vec()
    }

    fn next(&self) -> Vec<(Point, bool)> {
        self.data
            .iter()
            .filter(|(point, _)| match self.conway {
                false => true,
                true => !self.is_corner(point),
            })
            .map(|(point, on)| {
                let neighbors = self.neighbors(point);
                let neighbor_count = neighbors
                    .iter()
                    .filter(|p| *self.data.get(p).unwrap())
                    .count();
                (
                    *point,
                    match on {
                        true => neighbor_count == 2 || neighbor_count == 3,
                        false => neighbor_count == 3,
                    },
                )
            })
            .collect_vec()
    }

    fn is_corner(&self, point: &Point) -> bool {
        let max_width = self.width - 1;
        let max_height = self.height - 1;
        match point {
            (0, 0) => true,
            (0, w) => *w == max_width,
            (h, 0) => *h == max_height,
            (h, w) => *h == max_height && *w == max_width,
        }
    }

    fn step(&mut self) {
        for (point, on) in self.next() {
            self.data.insert(point, on);
        }
    }

    fn contains(&self, (row, column): Point) -> bool {
        0 <= row && row < self.height && 0 <= column && column < self.width
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = (0..self.height)
            .cartesian_product(0..self.height)
            .map(|p| {
                (
                    p,
                    match self.data.get(&p).unwrap() {
                        true => '#',
                        false => '.',
                    },
                )
            })
            .group_by(|((r, _), _)| *r)
            .into_iter()
            .map(|(_, group)| String::from_iter(group.into_iter().map(|(_, c)| c)))
            .collect_vec();
        write!(f, "{}", lines.join("\n"))
    }
}
#[cfg(test)]
mod test {
    use crate::day18::grid_steps;

    const EXAMPLE: &str = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn example() {
        assert_eq!(4, grid_steps(EXAMPLE, 4, false));
    }

    #[test]
    fn example2() {
        assert_eq!(17, grid_steps(EXAMPLE, 5, true));
    }
}
