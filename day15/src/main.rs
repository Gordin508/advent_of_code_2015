#![allow(unused)]
#![allow(dead_code)]

use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CookieProperties {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

impl CookieProperties {
    fn value(&self) -> i64 {
        if self.capacity <= 0 || self.durability <= 0 || self.flavor <= 0 || self.texture <= 0 {
            return 0;
        }
        self.capacity * self.durability * self.flavor * self.texture
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ingredient {
    properites: CookieProperties
}

impl From<&str> for CookieProperties {
    fn from(line: &str) -> Self {
        let nums = line.split_whitespace()
                       .filter_map(|n| n.trim_end_matches(',').parse::<i64>().ok())
                       .collect::<Vec<_>>();
        CookieProperties { capacity: nums[0], durability: nums[1], flavor: nums[2],
                           texture: nums[3], calories: nums[4] }
    }
}

impl Mul<i64> for CookieProperties {
    type Output = CookieProperties;

    fn mul(self, rhs: i64) -> Self::Output {
        CookieProperties { capacity: self.capacity * rhs, durability: self.durability * rhs,
                           flavor: self.flavor * rhs, texture: self.texture * rhs,
                           calories: self.calories * rhs }
    }
}

impl Add for CookieProperties {
    type Output = CookieProperties;

    fn add(self, rhs: Self) -> Self::Output {
        CookieProperties { capacity: self.capacity + rhs.capacity, durability: self.durability + rhs.durability,
                           flavor: self.flavor + rhs.flavor, texture: self.texture + rhs.texture,
                           calories: self.calories + rhs.calories }
    }
}

use std::iter::Sum;
impl Sum for CookieProperties {
    fn sum<I: Iterator<Item=CookieProperties>>(iter: I) -> Self {
        iter.fold(CookieProperties { capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0 }, |a, b| a + b)
    }
}

impl From<&str> for Ingredient {
    fn from(line: &str) -> Self {
        Ingredient { properites: CookieProperties::from(line) }
    }
}

use std::cmp::max;
fn bestcookie(ingredients: &[Ingredient], max_teaspoons: usize, calorie_target: Option<i64>) -> i64 {
    // brute-force all possible combinations
    let num_ingredients = ingredients.len();
    let mut separators = vec![0usize; ingredients.len() - 1];
    let mut teaspoons = vec![0usize; num_ingredients];
    teaspoons[0] = max_teaspoons;
    // **|****|*|***
    let mut best = 0;
    loop {
        assert!(teaspoons.iter().sum::<usize>() == max_teaspoons); //invariant
        let cookie = teaspoons.iter().zip(ingredients).map(|(n, ingr)| ingr.properites * *n as i64).sum::<CookieProperties>();
        best = match(calorie_target) {
            None => max(best, cookie.value()),
            Some(calories) => if cookie.calories == calories {max(best, cookie.value())} else {best}
        };

        // very inelegant way of creating all possible combinations
        // iterate separators
        for i in (0..num_ingredients - 1) {
            if separators[i] != max_teaspoons {
                separators[i] += 1;
                break;
            } else if i < num_ingredients - 2 {
                // carry
                separators[i] = 0;
            } else {
                // done
                return best;
            }
        }
        // restore order
        for i in (0..num_ingredients - 2).rev() {
            separators[i] = max(separators[i], separators[i + 1]);
        }
        // renew teaspoons
        for i in (0..num_ingredients) {
            if i == 0 {
                teaspoons[i] = max_teaspoons - separators[0];
            } else if i == num_ingredients - 1 {
                teaspoons[i] = separators[num_ingredients - 2]
            } else {
                teaspoons[i] = separators[i - 1] - separators[i];
            }
        }
    }
    best
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let ingredients: Vec<Ingredient> = lines.iter().map(|l| Ingredient::from(*l)).collect();
    Some(bestcookie(&ingredients, 100, None))
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let ingredients: Vec<Ingredient> = lines.iter().map(|l| Ingredient::from(*l)).collect();
    Some(bestcookie(&ingredients, 100, Some(500)))
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
