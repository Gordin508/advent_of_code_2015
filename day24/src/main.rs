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
    pos: usize,
    lastassigned: bool,
    entanglement: usize,
    front_presents: usize,
    forward: bool
}

impl DFSFrame {
    fn new(pos: usize, lastassigned: bool, entanglement: usize, front_presents: usize, forward: bool) -> DFSFrame {
        DFSFrame{pos, lastassigned, entanglement, front_presents, forward }
    }
}

fn has_valid_combination(presents: &[usize], assigned: &mut [bool], weights: &[usize], target_weight: usize) -> bool {
    assert!(assigned.len() == presents.len());
    if weights.len() == 0 {
        return true;
    } else if weights.len() == 1 && weights[0] == target_weight {
        return true;
    }
    assert!(weights[0] < target_weight);
    let missing = target_weight - weights[0];
    let unassigned: Vec<usize> = (0..presents.len()).filter(|n| !assigned[*n] && presents[*n] <= missing).collect();
    for n in unassigned {
        assigned[n] = true;
        let present = presents[n];
        let mut nweights = if present == missing { weights[1..].to_vec() } else { weights.to_vec() };
        if present != missing {
            nweights[0] += present;
        }
        assert!(nweights.len() == 0 || nweights[0] < target_weight);
        if has_valid_combination(presents, assigned, &nweights, target_weight) {
            return true;
        }
        assigned[n] = false;
    }

    false
}

fn balance(presents: &[usize], num_groups: usize) -> usize {
    let total = presents.iter().sum::<usize>();
    assert!(total % num_groups == 0);
    let partition_weight = total / num_groups;

    let mut weight = 0usize;
    let mut assigned = vec![false; presents.len()];
    let mut least_qe = usize::MAX;
    let mut least_presents = usize::MAX;
    let mut qe_upper_bound = 1usize;
    let mut lp_upper_bound = 0usize;
    let mut stack: Vec<DFSFrame> = Vec::new();
    stack.push(DFSFrame::new(0, false, 1, 0, true));
    stack.push(DFSFrame::new(0, true, 1, 0, true));

    while let Some(frame) = stack.pop() {
        if frame.forward {
            if frame.front_presents > least_presents
               || (frame.front_presents == least_presents && frame.entanglement >= least_qe) {
                continue;
            }

            if frame.lastassigned {
                weight += presents[frame.pos];
                assigned[frame.pos] = true;
            }

            assert!(weight <= partition_weight);
            // push backtrack frame
            let mut framecopy = frame.clone();
            framecopy.forward = false;
            stack.push(framecopy);

            // check win condition
            if weight == partition_weight {
                if (frame.front_presents < least_presents || frame.entanglement < least_qe) {
                    let remaining_presents: Vec<usize> = (0..presents.len()).filter(|n| !assigned[*n]).map(|n| presents[n]).collect();
                    let mut assigned = vec![false; remaining_presents.len()];
                    let weights = vec![0; num_groups - 1];
                    if has_valid_combination(&remaining_presents, &mut assigned, &weights, partition_weight) {
                        least_presents = frame.front_presents;
                        least_qe = frame.entanglement;
                    }
                }
                continue;
            }

            assert!(weight < partition_weight);
            let newpos = frame.pos + 1;
            if newpos < presents.len() {
                let present = presents[newpos];
                stack.push(DFSFrame::new(newpos, false, frame.entanglement, frame.front_presents, true));
                if weight + present <= partition_weight {
                    let qe_test = frame.entanglement.checked_mul(present);
                    if let Some(qe) = qe_test {
                        let fp = frame.front_presents + 1;
                        stack.push(DFSFrame::new(newpos, true, qe, fp, true));
                    }
                }
            }
        } else {
            if frame.lastassigned {
                assert!(assigned[frame.pos] == true);
                assigned[frame.pos] = false;
                weight -= presents[frame.pos];
            }
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
