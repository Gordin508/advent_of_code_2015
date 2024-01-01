#![allow(unused)]
#![allow(dead_code)]
use std::fs;
use std::env;


fn mine(puzzle_input: &[u8], nonce: usize) -> md5::Digest {
    let mut input = puzzle_input.to_vec();
    input.extend(nonce.to_string().as_bytes());
    md5::compute(&input)
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    // puzzle input is just a single line, but we need it as bytes
    let puzzle_input = lines[0].as_bytes();
    // iterate over all numbers i starting at 1
    for i in 1.. {
        let digest = mine(puzzle_input, i);
        // check if the digest starts with 5 zeros
        if (digest[0] | digest[1] | (digest[2] & 0xf0)) == 0 {
            return Some(i as i64)
        }
    }
    panic!("We should never reach this point");
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    // puzzle input is just a single line, but we need it as bytes
    let puzzle_input = lines[0].as_bytes();
    // iterate over all numbers i starting at 1
    for i in 1.. {
        let digest = mine(puzzle_input, i);
        // check if the digest starts with 5 zeros
        if (digest[0] | digest[1] | digest[2]) == 0 {
            return Some(i as i64)
        }
    }
    panic!("We should never reach this point");
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
