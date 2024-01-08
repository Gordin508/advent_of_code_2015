#![allow(unused)]
#![allow(dead_code)]


// return length of the resulting number
fn look_and_say(num: &str, rounds: usize) -> usize {
    // convert input str to vector of i8 (each char is a digit)
    let mut current: Vec<i8> = num.chars().map(|c| c.to_digit(10).unwrap() as i8).collect();
    for _ in (0..rounds) {
        current.push(10); // guard, spares us from handling the end separately
        let mut newnum: Vec<i8> = Vec::new();
        assert_ne!(current.len(), 0);
        let mut digit = current[0];
        let mut digitcount = 1;
        for next_digit in &current[1..current.len()] {
            if *next_digit == digit {
                digitcount += 1;
            } else {
                newnum.push(digitcount);
                newnum.push(digit);
                digit = *next_digit;
                digitcount = 1;
            }
        }
        current = newnum;
    }
    current.len()
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(look_and_say(lines[0], 40) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(look_and_say(lines[0], 50) as i64)
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
    static TESTINPUT: &str = "1113122113";


    #[test]
    fn test_lookandsay_3rounds() {
        assert_eq!(6, look_and_say(&1.to_string(), 5));
    }

    #[test]
    fn test_part1_1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(360154), part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(5103798), part2(&lines));
    }
}
