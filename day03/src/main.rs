#![allow(unused)]
#![allow(dead_code)]
use std::fs;
use std::env;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position{x, y}
    }

    fn walk(self, dir: char) -> Position {
        match(dir) {
            '^' => Position::new(self.x, self.y - 1),
            '>' => Position::new(self.x + 1, self.y),
            '<' => Position::new(self.x - 1, self.y),
            'v' => Position::new(self.x, self.y + 1),
            _ => panic!("Unexpected char!")
        }
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let height = lines.len();
    let width = lines[0].len();
    let mut presents: HashMap<Position, i64> = HashMap::new();
    let mut position = Position{x: 0, y: 0};
    // set presents value at Position(x, y) to 1
    presents.insert(position, 1);
    for line in lines.iter() {
        line.chars().for_each(|c| {
            position = position.walk(c);
            *presents.entry(position).or_insert(0) += 1;
        });
    }
        
    Some(presents.values().count() as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let height = lines.len();
    let width = lines[0].len();
    let mut presents: HashMap<Position, i64> = HashMap::new();
    let mut position: [Position; 2] = [Position{x: 0, y: 0}; 2];
    // set presents value at Position(x, y) to 1
    presents.insert(position[0], 2);
    let mut cindex: u8 = 0;
    for line in lines.iter() {
        line.chars().for_each(|c| {
            let index = cindex as usize;
            cindex = 1 - cindex;
            position[index] = position[index].walk(c);
            *presents.entry(position[index]).or_insert(0) += 1;
        });
    }
        
    Some(presents.values().count() as i64)
}

fn main() {
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
        let result = part(&lines);
        match result {
            Some(result) => println!("Part {}: {}", index+1, result),
            None => println!("Part {}: No result", index+1),
        }
    }
}
