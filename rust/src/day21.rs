use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32 as u32_nom};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::iter::Sum;
use std::ops::Add;

#[aoc_generator(day21)]
fn parse_boss(input: &str) -> Role {
    boss(input).unwrap().1
}

fn boss(input: &str) -> IResult<&str, Role> {
    map(
        tuple((
            delimited(tag("Hit Points: "), u32_nom, line_ending),
            delimited(tag("Damage: "), u32_nom, line_ending),
            preceded(tag("Armor: "), u32_nom),
        )),
        |(hp, damage, armor)| Role { hp, damage, armor },
    )(input)
}

#[aoc(day21, part1)]
fn solve_part1(boss: &Role) -> u32 {
    item_combinations()
        .into_iter()
        .map(|item| (item.cost, Role::from_item(item.clone())))
        .filter(|(_, role)| battle(role.clone(), boss.clone()))
        .map(|(cost, _)| cost)
        .min()
        .unwrap_or(0)
}

#[aoc(day21, part2)]
fn solve_part2(boss: &Role) -> u32 {
    item_combinations()
        .into_iter()
        .map(|item| (item.cost, Role::from_item(item.clone())))
        .filter(|(_, role)| !battle(role.clone(), boss.clone()))
        .map(|(cost, _)| cost)
        .max()
        .unwrap_or(0)
}

fn battle(this: Role, other: Role) -> bool {
    let mut player = this.clone();
    let mut boss = other.clone();
    while player.hp > 0 && boss.hp > 0 {
        boss.defend(&player);
        if boss.hp == 0 {
            break;
        }
        player.defend(&boss);
    }
    boss.hp == 0
}

///
/// ```text
/// Weapons:    Cost  Damage  Armor
/// Dagger        8     4       0
/// Shortsword   10     5       0
/// Warhammer    25     6       0
/// Longsword    40     7       0
/// Greataxe     74     8       0
///
/// Armor:      Cost  Damage  Armor
/// Leather      13     0       1
/// Chainmail    31     0       2
/// Splintmail   53     0       3
/// Bandedmail   75     0       4
/// Platemail   102     0       5
///
/// Rings:      Cost  Damage  Armor
/// Damage +1    25     1       0
/// Damage +2    50     2       0
/// Damage +3   100     3       0
/// Defense +1   20     0       1
/// Defense +2   40     0       2
/// Defense +3   80     0       3
/// ```
fn item_combinations() -> Vec<ShopItem> {
    let weapons = [
        ShopItem::from(8, 4, 0),
        ShopItem::from(10, 5, 0),
        ShopItem::from(25, 6, 0),
        ShopItem::from(40, 7, 0),
        ShopItem::from(74, 8, 0),
    ];
    let armor = [
        ShopItem::from(13, 0, 1),
        ShopItem::from(31, 0, 2),
        ShopItem::from(53, 0, 3),
        ShopItem::from(75, 0, 4),
        ShopItem::from(102, 0, 5),
    ];
    let rings = [
        ShopItem::from(25, 1, 0),
        ShopItem::from(50, 2, 0),
        ShopItem::from(100, 3, 0),
        ShopItem::from(20, 0, 1),
        ShopItem::from(40, 0, 2),
        ShopItem::from(80, 0, 3),
    ];

    let ring_choices = [None].into_iter().chain(rings.map(Some));
    iproduct!(
        weapons.into_iter().map(Some),
        [None].into_iter().chain(armor.map(Some)),
        ring_choices.clone(),
        ring_choices
    )
    .map(|(w, a, r1, r2)| vec![w, a, r1, r2].into_iter().unique().flatten().sum())
    .collect_vec()
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct ShopItem {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl ShopItem {
    fn empty() -> Self {
        ShopItem {
            cost: 0,
            damage: 0,
            armor: 0,
        }
    }
    fn from(cost: u32, damage: u32, armor: u32) -> Self {
        ShopItem {
            cost,
            damage,
            armor,
        }
    }
}

impl Add for ShopItem {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ShopItem {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

impl Sum for ShopItem {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap_or(ShopItem::empty())
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Role {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Role {
    fn from_item(item: ShopItem) -> Self {
        Role {
            hp: 100,
            damage: item.damage,
            armor: item.armor,
        }
    }

    fn defend(&mut self, other: &Role) {
        let damage = other.damage.saturating_sub(self.armor).max(1);
        self.hp = self.hp.saturating_sub(damage);
    }
}
#[cfg(test)]
mod test {
    use crate::day21::{battle, item_combinations, Role};

    #[test]
    fn wargames() {
        let player = Role {
            hp: 8,
            damage: 5,
            armor: 5,
        };
        let boss = Role {
            hp: 12,
            damage: 7,
            armor: 2,
        };
        assert_eq!(true, battle(player, boss));
    }

    #[test]
    fn item_combos() {
        let expected = 5 * 6 * 7 * 7;
        assert_eq!(expected, item_combinations().len())
    }
}
