#![allow(unused)]
#![allow(dead_code)]

use std::cmp::{min, max};

fn parse_presents(lines: &[&str]) -> Vec<usize> {
    lines.iter().map(|l| l.parse::<usize>().unwrap()).collect()
}

fn quantum_entanglement(presents: &[usize]) -> usize {
    presents.iter().product()
}

#[derive(Debug, Clone)]
struct DFSFrame {
    lastassigned: usize,
    assigned_to: usize,
    entanglement: usize,
    front_presents: usize,
    forward: bool
}

impl DFSFrame {
    fn new(lastassigned: usize, assigned_to: usize, entanglement: usize, front_presents: usize, forward: bool) -> DFSFrame {
        DFSFrame {lastassigned, assigned_to, entanglement, front_presents, forward }
    }
}

fn balance(presents: &[usize], num_groups: usize) -> usize {
    let total = presents.iter().sum::<usize>();
    assert!(total % num_groups == 0);
    let third = total / num_groups;

    let mut weights = vec![0usize; num_groups];
    let mut assignments = vec![0; presents.len()];
    let mut least_qe = usize::MAX;
    let mut least_presents = usize::MAX;
    let mut qe_upper_bound = 1usize;
    let mut lp_upper_bound = 0usize;
    let mut stack: Vec<DFSFrame> = Vec::new();
    stack.push(DFSFrame::new(0, 0, 1, 0, true));

    while let Some(frame) = stack.pop() {
        if frame.forward {
            if frame.front_presents > least_presents
               || (frame.front_presents == least_presents && frame.entanglement >= least_qe) {
                continue;
            }

            if frame.assigned_to > 0 {
                weights[frame.assigned_to - 1] += presents[frame.lastassigned];
                assignments[frame.lastassigned] = frame.assigned_to;
            }
            assert!(*weights.iter().max().unwrap() <= third);
            // push backtrack frame
            let mut framecopy = frame.clone();
            framecopy.forward = false;
            stack.push(framecopy);

            // check win condition
            if weights.iter().all(|w| *w == third) {
                if frame.front_presents < least_presents || frame.entanglement < least_qe {
                    least_presents = frame.front_presents;
                    least_qe = frame.entanglement;
                }
                continue;
            }

            let (i, _) = weights.iter().enumerate().find(|(i, w)| **w < third).unwrap();
            for j in assignments.iter().enumerate().filter(|(j, a)| **a == 0).map(|(j, a)| j) {
                let present = presents[j];
                if weights[i] + present > third {
                    continue;
                }
                let qe = if i == 0 {frame.entanglement * present} else {frame.entanglement};
                let fp = if i == 0 {frame.front_presents + 1} else {frame.front_presents};
                let mut newframe = DFSFrame::new(j, i + 1, qe, fp, true);
                stack.push(newframe);
            }
        } else {
            let index = frame.assigned_to;
            assert!(assignments[frame.lastassigned] == index);
            if index == 0 {
                continue;
            }
            assignments[frame.lastassigned] = 0;
            let present = presents[frame.lastassigned];
            weights[index - 1] -= present;
        }
    }

    least_qe
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let presents = parse_presents(lines);
    let qe = balance(&presents, 3);
    Some(qe as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let presents = parse_presents(lines);
    let qe = balance(&presents, 4);
    Some(qe as i64)
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
