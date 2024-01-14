#![allow(unused)]
#![allow(dead_code)]

fn simulate_machine(instructions: &[&str], startregs: &[usize; 2]) -> i64 {
    let mut rip: usize = 0;
    let mut regs = startregs.clone();
    let end = instructions.len();
    while (rip < end) {
        let split = instructions[rip].split_whitespace().collect::<Vec<_>>();
        if split[0] == "jmp" {
            let offset = split[1].parse::<isize>().unwrap();
            rip = if offset >= 0 { rip + offset as usize} else { rip - offset.abs() as usize };
            continue;
        }
        let register = match(split[1].trim_end_matches(',')) {
            "a" => 0,
            "b" => 1,
            _ => panic!("Unknown register")
        };
        match split[0] {
            "hlf" => {
                regs[register] /= 2;
            },
            "tpl" => {
                regs[register] *= 3;
            },
            "inc" => {
                regs[register] += 1;
            },
            "jie" => {
                if regs[register] % 2 == 0 {
                    let offset = split[2].parse::<isize>().unwrap();
                    rip = if offset >= 0 { rip + offset as usize} else { rip - offset as usize };
                    continue;
                }
            },
            "jio" => {
                if regs[register] == 1 {
                    let offset = split[2].parse::<isize>().unwrap();
                    rip = if offset >= 0 { rip + offset as usize} else { rip - offset as usize };
                    continue;
                }
            },
            _ => panic!("Unknown command")
        };
        rip += 1;
    }
    regs[1] as i64
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(simulate_machine(lines, &[0; 2]))
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(simulate_machine(lines, &[1, 0]))
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
