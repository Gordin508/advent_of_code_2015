#![allow(unused)]
#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::cmp::{max, min};

#[derive(Debug, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Effect {
    until: usize,  // time step
    heal: isize,
    damage: isize,
    armor: isize,
    mana_gain: isize,
    spell_type: Spell
}

impl std::cmp::Ord for Effect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.until.cmp(&self.until)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Boss {
    hp: isize,
    damage: isize
}

impl Boss {
    fn attack(&self, player: &mut Player) {
        player.hp -= max(1, self.damage - player.get_armor());
    }
}

impl From<&Vec<&str>> for Boss {
    fn from(lines: &Vec<&str>) -> Self {
        let parsenum = |line: &str| -> isize {
            line.split_whitespace().find_map(|w| w.parse::<isize>().ok()).unwrap()
        };
        Boss{hp: parsenum(lines[0]), damage: parsenum(lines[1])}
    }
}

#[derive(Debug)]
struct Player {
    hp: isize,
    mana: isize,
    effects: BinaryHeap<Effect>
}

impl Clone for Player {
    fn clone(&self) -> Self {
        let mut neweffects = BinaryHeap::new();
        for effect in &self.effects {
            neweffects.push(effect.clone());
        }
        Player{hp: self.hp, mana: self.mana, effects: neweffects}
    }
}

impl Player {
    fn heal(&mut self) {
        self.hp += self.get_heal();
    }
    fn attack(&self, boss: &mut Boss) {
        boss.hp -= self.get_damage();
    }
    fn update_effects(&mut self, step: usize) {
        while let Some(effect) = self.effects.peek() {
            if effect.until <= step {
                self.effects.pop();
            } else {
                break;
            }
        }
    }

    fn get_damage(&self) -> isize {
        self.effects.iter().fold(0, |acc, e| acc + e.damage)
    }

    fn get_armor(&self) -> isize {
        self.effects.iter().fold(0, |acc, e| acc + e.armor)
    }

    fn get_heal(&self) -> isize {
        self.effects.iter().fold(0, |acc, e| acc + e.heal)
    }

    fn get_manaregen(&self) -> isize {
        self.effects.iter().fold(0, |acc, e| acc + e.mana_gain)
    }
}

#[derive(Debug, Clone)]
struct DFSFrame {
    player: Player,
    boss: Boss,
    step: usize,
    mana_spent: isize,
}

impl DFSFrame {
    fn new(player: Player, boss: Boss, step: usize, mana_spent: isize) -> DFSFrame {
        DFSFrame{player, boss, mana_spent, step}
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Spell {
    MAGICMISSILE,
    DRAIN,
    SHIELD,
    POISON,
    RECHARGE
}

impl Spell {
    fn cost(&self) -> isize {
        match self {
            Spell::MAGICMISSILE => 53,
            Spell::DRAIN => 73,
            Spell::SHIELD => 113,
            Spell::POISON => 173,
            Spell::RECHARGE => 229
        }
    }

    fn cast(&self, player: &mut Player, boss: &mut Boss, step: usize) {
        assert!(player.mana >= self.cost());
        player.mana -= self.cost();
        match self {
            Spell::MAGICMISSILE => {
                boss.hp -= 4;
            },
            Spell::DRAIN => {
                boss.hp -= 2;
                player.hp += 2;
            },
            Spell::SHIELD => {
                player.effects.push(Effect{until: step + 6, heal: 0, damage: 0, armor: 7, mana_gain: 0, spell_type: *self});
            },
            Spell::POISON => {
                player.effects.push(Effect{until: step + 6, heal: 0, damage: 3, armor: 0, mana_gain: 0, spell_type: *self});
            },
            Spell::RECHARGE => {
                player.effects.push(Effect{until: step + 5, heal: 0, damage: 0, armor: 0, mana_gain: 101, spell_type: *self});
            }
        };
        player.update_effects(step + 1);
        player.mana += player.get_manaregen();
        player.hp += player.get_heal();
        player.attack(boss);
        boss.attack(player);
    }
}

fn find_best_strategy(player: &Player, boss: &Boss) -> isize {
    let mut best: isize = isize::MAX;
    use std::collections::VecDeque;
    let mut queue  = VecDeque::new();
    queue.push_back(DFSFrame::new(player.clone(), boss.clone(), 0, 0));
    while let Some(frame) = queue.pop_front() {
        if frame.mana_spent >= best {
            continue;
        }
        // someone died in last step?
        if frame.boss.hp <= 0 {
            println!("Attack kill: Turn: {}, Player HP: {}, Boss HP: {}, spent: {}", frame.step, frame.player.hp, frame.boss.hp, frame.mana_spent);
            println!("{:?}", frame.player.effects);
            best = min(best, frame.mana_spent);
            continue;
        } else if frame.player.hp <= 0 {
            continue;
        }
        let mut nplayer = frame.player.clone();
        let mut nboss = frame.boss.clone();
        let nextstep = frame.step + 2;
        nplayer.update_effects(frame.step);
        nplayer.mana += frame.player.get_manaregen();
        nplayer.hp += frame.player.get_heal();
        nplayer.attack(&mut nboss);
        if nboss.hp <= 0 {
            println!("DOT kill: Turn: {}, Player HP: {}, Boss HP: {}, spent: {}", frame.step, frame.player.hp, nboss.hp, frame.mana_spent);
            println!("{:?}", nplayer.effects);
            best = min(best, frame.mana_spent);
            continue;
        }
        for spell in [Spell::MAGICMISSILE, Spell::DRAIN, Spell::POISON, Spell::SHIELD, Spell::RECHARGE].iter() {
            if spell.cost() > frame.player.mana {
                continue;
            }
            if nplayer.effects.iter().any(|e| e.spell_type == *spell) {
                continue;
            }
            let mut nextboss = nboss.clone();
            let mut nextplayer = nplayer.clone();
            spell.cast(&mut nextplayer, &mut nextboss, frame.step);
            queue.push_back(DFSFrame::new(nextplayer, nextboss, nextstep, frame.mana_spent + spell.cost()));
        }
    }
    best
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let player = Player{hp: 50, mana: 500, effects: BinaryHeap::new()};
    let boss = Boss::from(lines);

    Some(find_best_strategy(&player, &boss) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    //TODO: implement me
    None
}

fn main() {
    use std::fs;
    use std::env;
    use std::time::Instant;
    let args: Vec<String> =  env::args().collect();
    let infile = args.get(1).unwrap_or_else(|| {
        println!("Usage: {} <puzzle input>", args[0]);
        std::process::exit(1);
    });

    let contents = fs::read_to_string(infile)
        .expect("Could not read in file");

    let lines: Vec<&str> = contents.lines().collect();

    // execute part 1 and part 2, print their results if they exist
    // later parts may follow, so we loop over the part functions
    let parts = [part1, part2];
    for (index, part) in parts.iter().enumerate() {
        let partstart = Instant::now();
        let result = part(&lines);
        match result {
            Some(result) => println!("Part {}: {}\t({:?})", index+1, result, partstart.elapsed()),
            None => println!("Part {}: No result", index+1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poison_missile() {
        let player = Player{hp: 10, mana: 250, effects: BinaryHeap::new()};
        let boss = Boss{hp: 13, damage: 8};
        assert_eq!(173 + 53, find_best_strategy(&player, &boss));
    }

    #[test]
    fn test_medium() {
        let player = Player{hp: 10, mana: 250, effects: BinaryHeap::new()};
        let boss = Boss{hp: 14, damage: 8};
        assert_eq!(229 + 113 + 73 + 173 + 53, find_best_strategy(&player, &boss));
    }

    #[test]
    #[ignore]
    fn test_part2() {
    }
}
