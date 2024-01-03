#![allow(unused)]
#![allow(dead_code)]

use std::ops::AddAssign;

use ndarray::{Array2, s};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(turn\son|turn\soff|toggle)\s(\d+),(\d+) through (\d+),(\d+).*").unwrap();
}

struct Instruction<'a> {
    action: &'a str,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

impl<'a> Instruction<'a> {
    fn parse(line: &'a str) -> Instruction {
        let caps = RE.captures(line).unwrap();
        let action = caps.get(1).unwrap().as_str();
        let x1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let x2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let y2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
        Instruction{action, x1, y1, x2, y2}
    }
}

fn parseinstruction(line: &str) {

}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let width = 1000;
    let height = 1000;
    // create height x width array of int8, initialized to 0
    let mut grid = Array2::<i8>::zeros((height, width));
    for line in lines {
        let instruction = Instruction::parse(line);
        if instruction.action != "toggle" {
            let newval = match instruction.action {
                "turn on" => 1,
                "turn off" => 0,
                _ => panic!("Unknown action {}", instruction.action),
            };
            grid.slice_mut(s![instruction.x1..=instruction.x2, instruction.y1..=instruction.y2]).fill(newval);
        } else {
            for x in instruction.x1..=instruction.x2 {
                for y in instruction.y1..=instruction.y2 {
                        grid[[x,y]] = 1 - grid[[x,y]];
                }
            }
        }
    }
    Some(grid.iter().fold(0, |acc, &x| acc + i64::from(x)))
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let width = 1000;
    let height = 1000;
    // create height x width array of int8, initialized to 0
    let mut grid = Array2::<i64>::zeros((height, width));
    let re = Regex::new(r"(turn\son|turn\soff|toggle)\s(\d+),(\d+) through (\d+),(\d+).*").unwrap();
    for line in lines {
        let instruction = Instruction::parse(line);
        let newval = match instruction.action {
            "turn on" => 1,
            "turn off" => -1,
            "toggle" => 2,
            _ => panic!("Unknown action {}", instruction.action),
        };
        if newval > 0 {
            grid.slice_mut(s![instruction.x1..=instruction.x2, instruction.y1..=instruction.y2]).add_assign(newval);
        } else {
            // each grid item needs to be clamped to 0 if negative
            for x in instruction.x1..=instruction.x2 {
                for y in instruction.y1..=instruction.y2 {
                        grid[[x,y]] = if grid[[x,y]] > -newval {grid[[x,y]] + newval} else { 0 };
                }
            }
        }
    }
    Some(grid.sum())
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
            Some(result) => println!("Part {}: {}\t({:.3?} s)", index+1, result, partstart.elapsed().as_secs_f64()),
            None => println!("Part {}: No result", index+1),
        }
    }
}
