#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;

static FACTS: &str = "children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

fn parse_facts(line: &str) -> HashMap<String, i64> {
    let mut line = line;
    if line.starts_with("Sue") {
        (_, line) = line.split_once(": ").unwrap()
    }
    let mut result = HashMap::new();
    for itm in line.split(", ") {
        let (key, value) = itm.split_once(": ").unwrap();
        result.insert(key.to_string(), value.parse().unwrap());
    }
    result
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let facts = parse_facts(FACTS);
    'lineloop: for (i, line) in lines.iter().enumerate() {
        let mfcsam = parse_facts(line);
        for (key, val) in mfcsam.iter() {
            if facts.get(key).unwrap_or_else(|| val) != val {
                continue 'lineloop;
            }
        }
        return Some((i + 1) as i64);
    }
    None
}
fn comparator(key: &str, val1: Option<&i64>, val2: i64) -> bool {
    if let Some(v1) = val1 {
        if key == "cats" || key == "trees" {
            return *v1 < val2;
        } else if  key == "pomeranians" || key == "goldfish" {
            return *v1 > val2;
        }
        return *v1 == val2
    }
    true
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let facts = parse_facts(FACTS);
    'lineloop: for (i, line) in lines.iter().enumerate() {
        let mfcsam = parse_facts(line);
        for (key, val) in mfcsam.iter() {
            if !comparator(key, facts.get(key), *val) {
                continue 'lineloop;
            }
        }
        return Some((i + 1) as i64);
    }
    None
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
    fn test_parse() {
        let facts = parse_facts(FACTS);
        assert_eq!(facts.get("children"), Some(&3));
        assert_eq!(facts.get("cats"), Some(&7));
        assert_eq!(facts.get("cars"), Some(&2));
        assert_eq!(facts.get("perfumes"), Some(&1));
    }
}
