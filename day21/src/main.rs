#![allow(unused)]
#![allow(dead_code)]


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Stats {
    damage: i64,
    armor: i64
}

use std::ops::{Add, Sub, AddAssign, SubAssign};

impl Add for Stats {
    type Output = Stats;

    fn add(self, other: Stats) -> Stats {
        Stats { damage: self.damage + other.damage, armor: self.armor + other.armor }
    }
}

impl Sub for Stats {
    type Output = Stats;

    fn sub(self, other: Stats) -> Stats {
        Stats { damage: self.damage - other.damage, armor: self.armor - other.armor }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, other: Stats) {
        *self = Stats { damage: self.damage + other.damage, armor: self.armor + other.armor };
    }
}

impl SubAssign for Stats {
    fn sub_assign(&mut self, other: Stats) {
        *self = Stats { damage: self.damage - other.damage, armor: self.armor - other.armor };
    }
}

// macro for creating a Stats struct from two i64s
macro_rules! stats {
    ($d:expr, $a:expr) => {
        Stats { damage: $d, armor: $a }
    };
}

#[derive(Debug, Clone, Copy)]
struct ShopItem {
    stats: Stats,
    cost: i64
}

impl Add for ShopItem {
    type Output = ShopItem;

    fn add(self, other: ShopItem) -> ShopItem {
        ShopItem { stats: self.stats + other.stats, cost: self.cost + other.cost }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entity {
    hp: i64,
    stats: Stats
}

macro_rules! shopitem {
    ($c:expr, $d:expr, $a:expr) => {
        ShopItem { cost: $c, stats: stats!($d, $a)}
    };
}

struct Shop {
    weapons: [ShopItem; 5],
    armor: [ShopItem; 6],
    rings: [ShopItem; 8]
}

impl Shop {
    fn new() -> Shop {
        Shop {
            weapons: [
                shopitem!(8, 4, 0),
                shopitem!(10, 5, 0),
                shopitem!(25, 6, 0),
                shopitem!(40, 7, 0),
                shopitem!(74, 8, 0),
            ],
            armor: [
                shopitem!(0, 0, 0), // dummy
                shopitem!(13, 0, 1),
                shopitem!(31, 0, 2),
                shopitem!(53, 0, 3),
                shopitem!(75, 0, 4),
                shopitem!(102, 0, 5),
            ],
            rings: [
                shopitem!(0, 0, 0), //dummy
                shopitem!(0, 0, 0), //dummy
                shopitem!(25, 1, 0),
                shopitem!(50, 2, 0),
                shopitem!(100, 3, 0),
                shopitem!(20, 0, 1),
                shopitem!(40, 0, 2),
                shopitem!(80, 0, 3),
            ]
        }
    }
}

impl From<&Vec<&str>> for Entity {
    fn from(lines: &Vec<&str>) -> Self {
        let parsenum = |line: &str| -> i64 {
            line.split_whitespace().find_map(|w| w.parse::<i64>().ok()).unwrap()
        };
        Entity{hp: parsenum(lines[0]), stats: Stats{damage: parsenum(lines[1]), armor: parsenum(lines[2])}}
    }
}

fn defeats(player: &Entity, boss: &Entity) -> bool {
    let player_dmg = (player.stats.damage - boss.stats.armor).max(1);
    let boss_dmg = (boss.stats.damage - player.stats.armor).max(1);
    let ttk = boss.hp / player_dmg + (if boss.hp % player_dmg > 0 { 1 } else { 0 });
    let alivetime = player.hp / boss_dmg + (if player.hp % boss_dmg > 0 { 1 } else { 0 });
    return ttk <= alivetime;
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let shop = Shop::new();
    let boss = Entity::from(lines);
    let player = Entity{hp: 100, stats: Stats{damage: 0, armor: 0}};
    let mut best = i64::MAX;
    for weapon in shop.weapons.iter() {
        for armor in shop.armor.iter() {
            for (i, ring1) in shop.rings.iter().enumerate() {
                for ring2 in shop.rings.iter().skip(i) {
                    let equipment = *weapon + *armor + *ring1 + *ring2;
                    let equipped_player = Entity{hp: player.hp, stats: equipment.stats};
                    let cost = equipment.cost;
                    if cost < best && defeats(&equipped_player, &boss) {
                        best = cost;
                    }
                }
            }
        }
    }
    Some(best)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let shop = Shop::new();
    let boss = Entity::from(lines);
    let player = Entity{hp: 100, stats: Stats{damage: 0, armor: 0}};
    let mut best = 0;
    for weapon in shop.weapons.iter() {
        for armor in shop.armor.iter() {
            for (i, ring1) in shop.rings.iter().enumerate() {
                for ring2 in shop.rings.iter().skip(i + 1) {
                    let equipment = *weapon + *armor + *ring1 + *ring2;
                    let equipped_player = Entity{hp: player.hp, stats: equipment.stats};
                    let cost = equipment.cost;
                    if cost > best && !defeats(&equipped_player, &boss) {
                        best = cost;
                    }
                }
            }
        }
    }
    Some(best)
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
