#![allow(unused)]
#![allow(dead_code)]

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let nums: Vec<usize> = lines[0].split_whitespace().filter_map(|w| w.trim_end_matches(',').trim_end_matches('.').parse::<usize>().ok()).collect();
    assert_eq!(2, nums.len());
    let row = nums[0];
    let column = nums[1];
    let mut index = 0;
    // project diagonally onto y axis
    let standardrow = row + column - 1;
    index = (standardrow * (standardrow - 1)) / 2 + column;
    const STARTCODE: usize = 20151125;
    let mut current = STARTCODE;
    for _ in (1..index) {
        current = (current * 252533) % 33554393;
    }
    Some(current as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    //TODO: implement me
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
    static TESTINPUT: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 1, column 3.";
    static TESTINPUT2: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 2, column 1.";

    #[test]
    fn test_part1_13() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(17289845), part1(&lines));
    }

    #[test]
    fn test_part1_21() {
        let lines: Vec<&str> = TESTINPUT2.lines().collect();
        assert_eq!(Some(31916031), part1(&lines));
    }
}
