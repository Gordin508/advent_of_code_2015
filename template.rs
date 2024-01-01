#![allow(unused)]
#![allow(dead_code)]
use std::fs;
use std::env;

fn part1(lines: &Vec<&str>) -> Option<i64> {
    //TODO: implement me
    None
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    //TODO: implement me
    None
}

fn main() {
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
