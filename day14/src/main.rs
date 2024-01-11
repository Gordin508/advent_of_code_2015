#![allow(unused)]
#![allow(dead_code)]

use std::{cmp::{min, max}, str::FromStr};

struct Reindeer {
    speed: usize,
    flightduration: usize,
    restduration: usize
}

impl From<&str> for Reindeer {
    fn from(line: &str) -> Self {
        let nums = line
                    .split_whitespace()
                    .filter_map(|w| w.parse::<usize>().ok())
                    .collect::<Vec<_>>();
        Reindeer{speed: nums[0], flightduration: nums[1], restduration: nums[2]}
    }
}

impl Reindeer {
    fn period(&self) -> usize {
        self.flightduration + self.restduration
    }

    fn get_distance(&self, time: usize) -> usize {
        self.speed * ((time / self.period()) * self.flightduration
                       + min(time % self.period(), self.flightduration))
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let mut best = 0;
    let duration = 2503;
    for reindeer in lines.iter().map(|l| Reindeer::from(*l)) {
        best = max(best, reindeer.get_distance(duration));
    }
    Some(best as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let mut best = 0;
    let duration = 2503;
    let reindeers: Vec<Reindeer> = lines.iter().map(|l| Reindeer::from(*l)).collect();
    let mut points = vec![0usize; reindeers.len()];
    let mut argmax = vec![0];
    for time in (1..duration) {
        argmax.clear();
        // brute force, no optimization
        let mut best = 0;
        for (i, dist) in reindeers.iter().map(|rd| rd.get_distance(time)).enumerate() {
            if dist > best {
                best = dist;
                argmax.clear();
                argmax.push(i);
            } else if dist == best {
                argmax.push(i);
            }
        }
        for i in argmax.iter() {
            points[*i] += 1;
        }
    }
    Some(*points.iter().max().unwrap() as i64)
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
