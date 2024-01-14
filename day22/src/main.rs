#![allow(unused)]
#![allow(dead_code)]

use std::cmp::{max, min};

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

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: isize,
    mana: isize,
    poisonduration: usize,
    armorduration: usize,
    manaduration: usize
}

impl Player {
    fn new(hp: isize, mana: isize) -> Player {
        Player{hp, mana, poisonduration: 0, armorduration: 0, manaduration: 0}
    }
    fn attack(&self, boss: &mut Boss) {
        boss.hp -= self.get_damage();
    }
    fn update_effects(&mut self) {
        if self.poisonduration > 0 {
            self.poisonduration -= 1;
        }
        if self.armorduration > 0 {
            self.armorduration -= 1;
        }
        if self.manaduration > 0 {
            self.manaduration -= 1;
        }
    }

    fn get_damage(&self) -> isize {
        if self.poisonduration > 0 { 3 } else { 0 }
    }

    fn get_armor(&self) -> isize {
        if self.armorduration > 0 { 7 } else { 0 }
    }

    fn get_manaregen(&self) -> isize {
        if self.manaduration > 0 { 101 } else { 0 }  
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
        match self {
            Spell::MAGICMISSILE => {
                boss.hp -= 4;
            },
            Spell::DRAIN => {
                boss.hp -= 2;
                player.hp += 2;
            },
            Spell::SHIELD => {
                assert!(player.armorduration == 0);
                player.armorduration = 6;
            },
            Spell::POISON => {
                assert!(player.poisonduration == 0);
                player.poisonduration = 6;
            },
            Spell::RECHARGE => {
                assert!(player.manaduration == 0);
                player.manaduration = 5;
            }
        };
        player.mana -= self.cost();
        player.mana += player.get_manaregen();
        player.attack(boss);
        player.update_effects();
        boss.attack(player);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Difficulty {
    NORMAL,
    HARD
}

fn find_best_strategy(player: &Player, boss: &Boss, difficulty: Difficulty) -> isize {
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
            best = min(best, frame.mana_spent);
            continue;
        } 
        let mut nplayer = frame.player.clone();
        if difficulty == Difficulty::HARD {
            nplayer.hp -= 1;
        }
        if frame.player.hp <= 0 {
            continue;
        }
        let mut nboss = frame.boss.clone();
        let nextstep = frame.step + 2;
        nplayer.mana += frame.player.get_manaregen();
        nplayer.attack(&mut nboss);
        nplayer.update_effects();
        if nboss.hp <= 0 {
            best = min(best, frame.mana_spent);
            continue;
        }
        for spell in [Spell::MAGICMISSILE, Spell::DRAIN, Spell::POISON, Spell::SHIELD, Spell::RECHARGE].iter() {
            if spell.cost() > frame.player.mana {
                continue;
            }
            if *spell == Spell::POISON && nplayer.poisonduration > 0
                || *spell == Spell::SHIELD && nplayer.armorduration > 0
                || *spell == Spell::RECHARGE && nplayer.manaduration > 0 {
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
    let player = Player::new(50, 500);
    let boss = Boss::from(lines);

    Some(find_best_strategy(&player, &boss, Difficulty::NORMAL) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let player = Player::new(50, 500);
    let boss = Boss::from(lines);

    Some(find_best_strategy(&player, &boss, Difficulty::HARD) as i64)
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
        let player = Player::new(10, 250);
        let boss = Boss{hp: 13, damage: 8};
        assert_eq!(173 + 53, find_best_strategy(&player, &boss, Difficulty::NORMAL));
    }

    #[test]
    fn test_medium() {
        let player = Player::new(10, 250);
        let boss = Boss{hp: 14, damage: 8};
        assert_eq!(229 + 113 + 73 + 173 + 53, find_best_strategy(&player, &boss, Difficulty::NORMAL));
    }
}
