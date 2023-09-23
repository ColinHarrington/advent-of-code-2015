use aoc_runner_derive::{aoc, aoc_generator};
use nom::character::complete::{char, u32 as nom_u32};
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{delimited, tuple};

#[aoc_generator(day2)]
fn parse_gifts(input: &str) -> Vec<Gift> {
    input.lines()
        .map(|line| gift(line).unwrap().1)
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(gifts: &[Gift]) -> u32 {
    gifts.iter()
        .map(|gift| gift.wrapping_paper()).sum()
}
#[aoc(day2, part2)]
pub fn solve_part2(gifts: &[Gift]) -> u32 {
    gifts.iter()
        .map(|gift| gift.ribbon()).sum()
}


fn gift(input: &str) -> IResult<&str, Gift> {
    map(tuple((nom_u32, delimited(char('x'), nom_u32, char('x')), nom_u32)),
        |(l, w, h)| Gift { l, w, h })(input)
}

#[derive(Debug)]
pub struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl Gift {
    fn surfaces(&self) -> [u32; 3] {
        [
            self.l * self.w,
            self.w * self.h,
            self.h * self.l
        ]
    }

    fn ribbon_wrap(&self) -> u32 {
        let mut sides= [self.l, self.w, self.h];
            sides.sort();
        sides.iter().take(2).sum::<u32>() * 2
    }
    fn ribbon_bow(&self) -> u32 {
         self.l * self.w * self.h
    }
    fn wrapping_paper(&self) -> u32 {
        self.surfaces().iter().min().unwrap() + self.surfaces().iter().map(|&surface| surface * 2).sum::<u32>()
    }

    fn ribbon(&self) -> u32 {
        self.ribbon_wrap() + self.ribbon_bow()
    }
}

#[cfg(test)]
mod test {
    use crate::day2::Gift;

    #[test]
    fn ribbon() {
        let gift = Gift{l:2, w:3, h:4};
        assert_eq!(10, gift.ribbon_wrap());
        assert_eq!(24, gift.ribbon_bow());
        assert_eq!(34, gift.ribbon());

        let gift2 = Gift{l:1, w:1, h:10};
        assert_eq!(4, gift2.ribbon_wrap());
        assert_eq!(10, gift2.ribbon_bow());
        assert_eq!(14, gift2.ribbon());
    }
}