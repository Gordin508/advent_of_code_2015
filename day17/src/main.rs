#![allow(unused)]
#![allow(dead_code)]

fn parse_buckets(lines: &Vec<&str>) -> Vec<i64> {
    lines.iter().map(|l| l.parse::<i64>()
         .expect("Malformed input line"))
         .collect()
}

fn combinations(buckets: &[i64], target_capacity: i64) -> usize {
    let num_buckets = buckets.len();
    if target_capacity == 0 {
        return 1;
    } else if target_capacity < 0 || num_buckets == 0 {
        return 0;
    }
    combinations(&buckets[1..num_buckets], target_capacity - buckets[0])
    + combinations(&buckets[1..num_buckets], target_capacity)
}

fn bounded_combinations(buckets: &[i64], target_capacity: i64, num_containers: usize) -> usize {
    let num_buckets = buckets.len();
    if target_capacity == 0  && num_containers == 0{
        return 1;
    } else if target_capacity < 0 || num_buckets == 0 || num_containers <= 0 {
        return 0;
    }
    bounded_combinations(&buckets[1..num_buckets], target_capacity - buckets[0], num_containers - 1)
    + bounded_combinations(&buckets[1..num_buckets], target_capacity, num_containers)
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let buckets = parse_buckets(lines);
    const TARGETCAPACITY: usize = 150;
    Some(combinations(&buckets, TARGETCAPACITY as i64) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let mut buckets = parse_buckets(lines);
    buckets.sort();
    const TARGETCAPACITY: usize = 150;
    let mut capacity: usize = 0;
    // minor optim: calculate the smallest possible number
    // of containers beforehand
    let mut smallest = usize::MAX;
    for (i, bucket) in buckets.iter().rev().enumerate() {
        capacity += *bucket as usize;
        if capacity >= TARGETCAPACITY {
            smallest = i + 1;
            break;
        }
    }
    Some(bounded_combinations(&buckets, TARGETCAPACITY as i64, smallest) as i64)
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
    static TESTINPUT: &str = "20\n15\n10\n5\n5";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        let buckets = parse_buckets(&lines);
        assert_eq!(4, combinations(&buckets, 25));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(13337), part2(&lines));
    }
}
