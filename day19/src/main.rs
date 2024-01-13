#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug, PartialOrd, Hash, PartialEq, Eq)]
pub struct Rule {
    input: String,
    output: String,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (input, output) = value.split_once(" => ").expect("Invalid input");
        Rule {input: input.to_string(), output: output.to_string()}
    }
}

//implement ordering for Rule solely based on the input
impl Ord for Rule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.input.cmp(&other.input)
    }
}

#[derive(Debug)]
pub struct Grammar {
    rules: Vec<Rule>
}

impl From<&Vec<&str>> for Grammar {
    fn from(lines: &Vec<&str>) -> Self {
        let mut rules = Vec::new();
        for line in lines.iter().filter(|l| l.contains(" => ")) {
            rules.push(Rule::from(*line));
        }
        rules.sort();
        Grammar {rules}
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let grammar = Grammar::from(lines);
    let inputstr = lines.last().unwrap();
    use std::collections::HashSet;
    let mut seen: HashSet<String> = HashSet::new();
    for (charpos, c) in inputstr.char_indices() {
        let (prefix, suffix) = inputstr.split_at(charpos);
        for rule in grammar.rules.iter().filter(|r| suffix.starts_with(&r.input)) {
            let mut result = prefix.to_owned();
            result.push_str(&rule.output);
            result.push_str(&suffix[rule.input.len()..]);
            seen.insert(result);
        }
    }
    Some(seen.len() as i64)
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct BFSBranch {
    depth: usize,
    molecule: String,
}

impl Ord for BFSBranch {
    // priority shall be inversely proportional to string length
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return (-(self.molecule.len() as isize)).cmp(&-(other.molecule.len() as isize));
    }
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let grammar = Grammar::from(lines);
    let medicine = lines.last().unwrap();
    let goal = "e";
    use std::collections::{HashSet, BinaryHeap};
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(BFSBranch{depth: 0, molecule: medicine.to_string()});
    'outer: while let Some(branch) = queue.pop() {
        for (charpos, c) in branch.molecule.char_indices() {
            let (prefix, suffix) = branch.molecule.split_at(charpos);
            for rule in grammar.rules.iter().filter(|r| suffix.starts_with(&r.output)) {
                let mut result = prefix.to_owned();
                result.push_str(&rule.input);
                result.push_str(&suffix[rule.output.len()..]);
                if result == goal {
                    return Some(branch.depth as i64 + 1);
                }
                if result.len() <= medicine.len() && seen.insert(result.clone()) {
                    // new molecule discovered
                    queue.push(BFSBranch{depth: branch.depth + 1, molecule: result});
                }
            }
        }
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
    static TESTINPUT: &str = "CHANGEME";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(1337), part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
