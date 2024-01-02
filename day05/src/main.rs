#![allow(unused)]
#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display};

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let mut result: i64 = 0;
    // sliding window
    const FORBIDDEN: [&[u8; 2]; 4] = [b"ab", b"cd", b"pq", b"xy"];
    'outer: for line in lines {
        let chars = line.as_bytes();
        let mut vowels: u32 = 0;
        let mut pair = false;
        for i in 0..chars.len() {
            match chars[i] as char {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => ()
            };
            if i < chars.len() - 1 {
                pair = pair || chars[i] == chars[i + 1];
                for forbidden in FORBIDDEN {
                    if *forbidden.as_ref() == chars[i..i + 2] {
                        continue 'outer;
                    }
                }
            }
        }

        if vowels >= 3 && pair {
            result += 1;
        }
    }
    Some(result)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    use std::collections::HashMap;
    let mut result = 0;

    // this is rather ugly, but it at least it's relatively efficient
    'outer: for line in lines {
        let chars: Vec<char> = line.as_bytes().iter().map(|c| *c as char).collect();

        // keep track of char tuples we found
        let mut tuples: HashMap<&[char], usize> = HashMap::new();

        // nice conditions
        let mut repeats = false;
        let mut doublepair = false;

        'sliding_window: for i in 0..chars.len() - 1 {

            // check for pattern aba
            if !repeats && i < chars.len() - 2 && chars[i] == chars[i + 2] {
                repeats = true;
            }

            if !doublepair {
                // check if current pair exists somewhere else
                if let Some(pos) = tuples.get(&chars[i..i + 2]) {

                    // check if non-overlapping
                    if *pos < i - 1 {
                        doublepair = true;
                    }
                } else {
                    // we only update if the key is not present,
                    // because the smallest pos value is always the relevant one
                    tuples.insert(&chars[i..i + 2], i);
                }
            }

            if repeats && doublepair {
                // exit early as there are not conditions which could 'unnice' us
                break 'sliding_window;
            }
        }

        if repeats && doublepair {
            result += 1;
        }
    }
    Some(result)
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
