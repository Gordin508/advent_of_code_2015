#![allow(unused)]
#![allow(dead_code)]
use std::fs;
use std::env;

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(lines.iter()
              .map(|line| {
                let mut dims: Vec<i64> = line.split('x')
                    .map(|dim| dim.parse::<i64>().expect("Unparsable"))
                    .collect();
                assert_eq!(3, dims.len());
                dims.sort();
                2 * (&dims[0] * (&dims[1] + &dims[2]) + &dims[1] * &dims[2]) + &dims[0] * &dims[1]
                }).sum())
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(lines.iter()
              .map(|line| {
                let mut dims: Vec<i64> = line.split('x')
                    .map(|dim| dim.parse::<i64>().expect("Unparsable"))
                    .collect();
                assert_eq!(3, dims.len());
                dims.sort();
                2 * (&dims[0] + &dims[1]) + dims.iter().product::<i64>()
                }).sum())
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
