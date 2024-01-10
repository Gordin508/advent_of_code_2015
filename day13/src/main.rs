#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;
type Happiness = isize;

struct HappinessGraph {
    guests: HashMap<String, usize>,
    changes: HashMap<(usize, usize), Happiness>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct DFSFrame {
    happiness: Happiness,
    num_seated: usize,
    forward: bool,
    last_seated: usize
}

impl DFSFrame {
    fn new(happiness: Happiness, num_seated: usize, last_seated: usize) -> Self {
        DFSFrame{happiness, num_seated, forward: true, last_seated}
    }

    fn backtrack(&self) -> Self {
        DFSFrame{happiness: self.happiness,
                 num_seated: self.num_seated,
                 forward: false,
                 last_seated: self.last_seated}
    }
}

impl HappinessGraph {
    fn get_change(&self, guest1: usize, guest2: usize) -> Happiness {
        self.changes.get(&(guest1, guest2)).unwrap_or(&0isize)
        + self.changes.get(&(guest2, guest1)).unwrap_or(&0isize)
    }

    fn best_total(&self, seat_self: bool) -> Happiness {
        let num_guests = if seat_self { self.guests.len() + 1 } else {self.guests.len()};
        if num_guests == 0 {
            return 0;
        }
        let mut best: Happiness = 0;
        let mut seated = vec![false; num_guests];
        seated[0] = true;
        let mut num_seated = 0;
        use std::collections::BinaryHeap;
        let mut stack = Vec::new();
        stack.push(DFSFrame::new(0, 0, 0));
        while let Some(frame) = stack.pop() {
            if frame.forward {
                stack.push(frame.backtrack());
                seated[frame.last_seated] = true;
                num_seated += 1;
                if num_seated == num_guests {
                    let total = frame.happiness + self.get_change(0, frame.last_seated);
                    best = if best >= total { best } else { total };
                    continue;
                }
                for neighbor in (0..num_guests).filter(|n| !seated[*n]) {
                    let change = self.get_change(frame.last_seated, neighbor);
                    stack.push(DFSFrame::new(frame.happiness + change, num_seated, neighbor));
                }
            } else {
                seated[frame.last_seated] = false;
                num_seated -= 1;
            }
        }
        best
    }
}

// parsing from lines
impl From<&Vec<&str>> for HappinessGraph {
    fn from(lines: &Vec<&str>) -> Self {
        let mut guests = HashMap::new();
        let mut changes = HashMap::new();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            let guest1 = words[0];
            let sign = if words[2] == "lose" { -1 } else { 1 };
            let mut happiness: Happiness = sign * words[3].parse::<Happiness>().unwrap();
            let guest2 = words.last().unwrap().trim_end_matches('.');
            let mut guests_len = guests.len();
            let guest1 = *guests.entry(guest1.to_string()).or_insert(guests_len);
            guests_len = guests.len();
            let guest2 = *guests.entry(guest2.to_string()).or_insert(guests_len);
            changes.insert((guest1, guest2), happiness);
        }
        HappinessGraph { guests, changes }
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(HappinessGraph::from(lines).best_total(false) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(HappinessGraph::from(lines).best_total(true) as i64)
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
    static TESTINPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(330), part1(&lines));
    }
}
