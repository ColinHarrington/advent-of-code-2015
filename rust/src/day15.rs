use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, i32 as i32_nom, line_ending, u32 as u32_nom};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;

#[aoc_generator(day15)]
fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    ingredients(input).unwrap().1
}
#[aoc(day15, part1)]
fn solve_part1(ingredients: &[Ingredient]) -> i32 {
    let mut best = 0;

    for i in 0..=100 {
        let rem1 = 100 - i;
        for j in 0..=rem1 {
            let rem2 = rem1 - j;
            for k in 0..=rem2 {
                let l = rem2 - k;
                best = best.max(score(
                    ingredients
                        .iter()
                        .zip([i, j, k, l].into_iter())
                        .map(|(ingredient, qty)| ingredient.scoop(qty))
                        .reduce(mix)
                        .unwrap(),
                ));
            }
        }
    }
    best
}

#[aoc(day15, part2)]
fn solve_part2(ingredients: &[Ingredient]) -> i32 {
    let mut best = 0;

    for i in 0..=100 {
        let rem1 = 100 - i;
        for j in 0..=rem1 {
            let rem2 = rem1 - j;
            for k in 0..=rem2 {
                let l = rem2 - k;
                let spoon = ingredients
                    .iter()
                    .zip([i, j, k, l].into_iter())
                    .map(|(ingredient, qty)| ingredient.scoop(qty))
                    .reduce(mix)
                    .unwrap();

                if spoon.4 == 500 {
                    let score = score(spoon);
                    if score > best {
                        best = score
                    }
                }
            }
        }
    }
    best
}

type Spoonful = (i32, i32, i32, i32, u32);

fn mix(spoon1: Spoonful, spoon2: Spoonful) -> Spoonful {
    (
        spoon1.0 + spoon2.0,
        spoon1.1 + spoon2.1,
        spoon1.2 + spoon2.2,
        spoon1.3 + spoon2.3,
        spoon1.4 + spoon2.4,
    )
}
fn score((capacity, durability, flavor, texture, _): Spoonful) -> i32 {
    if [capacity, durability, flavor, texture]
        .iter()
        .all(|&v| v >= 0)
    {
        capacity * durability * flavor * texture
    } else {
        0
    }
}
fn ingredients(input: &str) -> IResult<&str, Vec<Ingredient>> {
    separated_list1(line_ending, ingredient)(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        tuple((
            terminated(alpha1::<&str, _>, tag(": ")),
            delimited(tag("capacity "), i32_nom, tag(", ")),
            delimited(tag("durability "), i32_nom, tag(", ")),
            delimited(tag("flavor "), i32_nom, tag(", ")),
            delimited(tag("texture "), i32_nom, tag(", ")),
            preceded(tag("calories "), u32_nom),
        )),
        |(_, capacity, durability, flavor, texture, calories)| Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )(input)
}
#[derive(Debug)]
pub struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

impl Ingredient {
    fn scoop(&self, spoons: i32) -> Spoonful {
        (
            self.capacity * spoons,
            self.durability * spoons,
            self.flavor * spoons,
            self.texture * spoons,
            self.calories * (spoons as u32),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::day15::{ingredient, mix, score};

    #[test]
    fn example_scoops_part1() {
        let butterscotch =
            ingredient("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8")
                .unwrap()
                .1;

        let cinnamon =
            ingredient("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3")
                .unwrap()
                .1;

        let mixed = mix(butterscotch.scoop(44), cinnamon.scoop(56));

        assert_eq!(62842880, score(mixed));
    }

    #[test]
    fn example_scoops_part2() {
        let butterscotch =
            ingredient("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8")
                .unwrap()
                .1;

        let cinnamon =
            ingredient("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3")
                .unwrap()
                .1;

        let mixed = mix(butterscotch.scoop(40), cinnamon.scoop(60));

        assert_eq!(500, mixed.4);
        assert_eq!(57600000, score(mixed));
    }

    #[test]
    fn example_ingredient() {
        let butterscotch =
            ingredient("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8")
                .unwrap()
                .1;
        assert_eq!(-1, butterscotch.capacity);
        assert_eq!(-2, butterscotch.durability);
        assert_eq!(6, butterscotch.flavor);
        assert_eq!(3, butterscotch.texture);
        assert_eq!(8, butterscotch.calories);
    }
}
