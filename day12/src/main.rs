#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;

use serde_json::{self, Value, Map};

fn sum_numbers_json(node: &Value) -> i64 {
    // recursively parse the nodes and sum up the results
    if node.is_i64() {
        return node.as_i64().unwrap();
    } else if node.is_object() {
        return sum_numbers_object(node.as_object().unwrap());
    } else if node.is_array() {
        return node.as_array().unwrap().iter().map(sum_numbers_json).sum::<i64>()
    }
    0
}

fn sum_numbers_object(object: &Map<String, Value>) -> i64 {
    if object.values().any(|v| v.as_str() == Some("red")) {
        return 0;
    }
    object.values().map(sum_numbers_json).sum::<i64>()
}

fn sum_numbers_ignorered(lines: &Vec<&str>) -> i64 {
    let parsed: serde_json::Value = serde_json::from_str(&lines.join("\n")).unwrap();
    sum_numbers_json(&parsed)
}

fn sum_numbers(lines: &Vec<&str>) -> i64 {
    // simple sliding window approach, a lot faster
    // than actually parsing the json
    let mut result = 0i64;
    for line in lines {
        let mut i = 0usize;
        let mut j = 0usize;
        for (pos, c) in line.chars().chain(['\n']).enumerate() {
            if !c.is_numeric() && c != '-' {
                if i < j {
                    result += match(line[i..j].parse::<i64>()) {
                        Ok(value) => value,
                        Err(err) => {println!("Error parsing {i}..{j}: {err}"); 0}
                    };
                }
                i = pos + 1;
                j = i;
            } else {
                j += 1;
            }
        }
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(sum_numbers(lines))
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(sum_numbers_ignorered(lines))
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
    fn sum_arr() {
        assert_eq!(6, sum_numbers(&vec!["[1,2,3]"]));
    }

    #[test]
    fn sum_collection() {
        assert_eq!(6, sum_numbers(&vec!["{\"a\":2,\"b\":4}"]));
    }

    #[test]
    fn sum_nested() {
        assert_eq!(3, sum_numbers(&vec!["[[[3]]]"]));
        assert_eq!(3, sum_numbers(&vec!["{\"a\":{\"b\":4},\"c\":-1}"]));
    }

    #[test]
    fn sum_empty() {
        assert_eq!(0, sum_numbers(&vec!["[]"]));
        assert_eq!(0, sum_numbers(&vec!["{}"]));
    }

    #[test]
    fn sum_negative() {
        assert_eq!(0, sum_numbers(&vec!["{\"a\":[-1,1]}"]));
        assert_eq!(0, sum_numbers(&vec!["-1,{\"a\":1}]"]));
    }

    #[test]
    fn sum_ignoreed_ignoreall() {
        assert_eq!(0, sum_numbers_ignorered(&vec!["{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"]));
    }

    #[test]
    fn sum_ignoreed_ignoreinner() {
        assert_eq!(4, sum_numbers_ignorered(&vec!["[1,{\"c\":\"red\",\"b\":2},3]"]));
    }
}
