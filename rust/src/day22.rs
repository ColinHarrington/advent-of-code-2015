use crate::day22::Spell::{Drain, MagicMissile, Poison, Recharge, Shield};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32 as u32_nom};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::collections::VecDeque;
use std::ops::Add;

#[aoc_generator(day22)]
fn parse_boss(input: &str) -> Boss {
    boss(input).unwrap().1
}

fn boss(input: &str) -> IResult<&str, Boss> {
    map(
        tuple((
            delimited(tag("Hit Points: "), u32_nom, line_ending),
            preceded(tag("Damage: "), u32_nom),
        )),
        |(hp, damage)| Boss { hp, damage },
    )(input)
}

#[aoc(day22, part1)]
fn solve_part1(boss: &Boss) -> u32 {
    let player = Player { hp: 50, mana: 500 };
    let start = GameState::new(player, boss, false);
    play(start)
}

#[aoc(day22, part2)]
fn solve_part2(boss: &Boss) -> u32 {
    let player = Player { hp: 50, mana: 500 };
    let start = GameState::new(player, boss, true);
    play(start)
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Boss {
    hp: u32,
    damage: u32,
}

struct Player {
    hp: u32,
    mana: u32,
}

#[derive(Debug, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    const VALUES: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];
    fn mana_cost(&self) -> u32 {
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct GameState {
    mana_spent: u32,
    mana: u32,
    hp: u32,
    boss_hp: u32,
    boss_damage: u32,
    recharge_timer: u8,
    shield_timer: u8,
    poison_timer: u8,
    difficult: bool,
}

impl GameState {
    fn new(player: Player, boss: &Boss, difficult: bool) -> Self {
        Self {
            mana_spent: 0,
            mana: player.mana,
            hp: player.hp,
            boss_hp: boss.hp,
            boss_damage: boss.damage,
            shield_timer: 0,
            recharge_timer: 0,
            poison_timer: 0,
            difficult,
        }
    }
    fn castable_spells(&self) -> Vec<&Spell> {
        Spell::VALUES
            .iter()
            .filter(|spell| match spell {
                Poison => self.poison_timer <= 1,
                Recharge => self.recharge_timer <= 1,
                Shield => self.shield_timer <= 1,
                _ => true,
            })
            .filter(|spell| spell.mana_cost() <= self.mana)
            .collect_vec()
    }

    fn play_round(&self, spell: &Spell) -> GameState {
        let mut state: GameState = *self;
        if state.difficult {
            state.hp = state.hp.saturating_sub(1);
        }
        if state.hp > 0 {
            state.effects();
            state.cast(spell);
            if state.boss_hp > 0 {
                state.effects();
                if state.boss_hp > 0 {
                    let armor = if state.shield_timer > 0 { 7 } else { 0 };
                    let damage = state.boss_damage.saturating_sub(armor);
                    state.hp = state.hp.saturating_sub(damage);
                }
            }
        }

        state
    }
    fn effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer = self.shield_timer.saturating_sub(1);
        }
        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer = self.recharge_timer.saturating_sub(1);
        }
        if self.poison_timer > 0 {
            self.poison_timer = self.poison_timer.saturating_sub(1);
            self.boss_hp = self.boss_hp.saturating_sub(3);
        }
    }

    fn cast(&mut self, spell: &Spell) {
        let mana_cost = spell.mana_cost();
        self.mana -= mana_cost;
        self.mana_spent += mana_cost;

        match spell {
            MagicMissile => self.boss_hp = self.boss_hp.saturating_sub(4),
            Drain => {
                self.boss_hp = self.boss_hp.saturating_sub(2);
                self.hp = self.hp.add(2)
            }
            Shield => self.shield_timer = 6,
            Poison => self.poison_timer = 6,
            Recharge => self.recharge_timer = 5,
        }
    }
}

fn play(start: GameState) -> u32 {
    let mut queue = VecDeque::from([start]);
    let mut min = u32::MAX;
    while let Some(state) = queue.pop_front() {
        if state.mana_spent <= min {
            if state.boss_hp == 0 {
                if min > state.mana_spent {
                    min = state.mana_spent;
                }
            } else if state.hp > 0 {
                state.castable_spells().into_iter().for_each(|spell| {
                    queue.push_back(state.play_round(spell));
                })
            }
        }
    }
    min
}

#[cfg(test)]
mod test {
    use crate::day22::Spell::{Drain, MagicMissile, Poison, Recharge, Shield};
    use crate::day22::{Boss, GameState, Player};

    #[test]
    fn first_example() {
        let player = Player { hp: 10, mana: 250 };
        let boss = Boss { hp: 13, damage: 8 };

        let game = [Poison, MagicMissile]
            .iter()
            .fold(GameState::new(player, &boss, false), |state, spell| {
                state.play_round(spell)
            });

        assert_eq!(0, game.boss_hp);
    }

    #[test]
    fn second_example() {
        let player = Player { hp: 10, mana: 250 };
        let boss = Boss { hp: 14, damage: 8 };

        let game = [Recharge, Shield, Drain, Poison, MagicMissile]
            .iter()
            .fold(GameState::new(player, &boss, false), |state, spell| {
                state.play_round(spell)
            });

        assert_eq!(0, game.boss_hp);
        assert_eq!(1, game.hp);
        assert_eq!(114, game.mana);
        assert_eq!(0, game.shield_timer);
    }
}
