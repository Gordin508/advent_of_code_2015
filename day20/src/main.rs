use std::collections::HashMap;

/*
 * This implementation is not ideal.
 * It takes around 35sec per part in debug
 * and 5 sec per part in release mode.
 */
fn factors(num: usize, known_factors: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if num == 0 {
        return Vec::new();
    } else if num == 1 {
        return vec![1];
    }
    if let Some(factors) = known_factors.get(&num) {
        return factors.clone();
    }
    let root = (num as f64).sqrt() as usize + 1;
    let mut fact = Vec::new();
    for i in (1usize..=root).rev() {
        if i == num {
            continue;
        }
        if num % i == 0 {
            fact.extend(factors(i, known_factors));
            if i == 1 {
                fact.push(num);
            } else {
                fact.extend(factors(num / i, known_factors));
            }
        }
    }
    let mut result = Vec::new();
    result.push(1);
    if fact.len() > 1 {
        fact.sort();
        for f in fact {
            if f != 1 && !result.iter().any(|s| *s != 1 && f == *s) {
                result.push(f);
            }
        }
    }
    known_factors.insert(num, result.clone());
    result
}

fn presents(house: usize, known_factors: &mut HashMap<usize, Vec<usize>>) -> usize {
    if house == 1 {
        return 10;
    }
    let factors = factors(house, known_factors);
    10 * (factors.iter().sum::<usize>())
}

fn presents_part2(house: usize, known_factors: &mut HashMap<usize, Vec<usize>>, max_iters: usize) -> usize {
    if house == 1 {
        return 11;
    }
    let mindiv = house / max_iters;
    11 * factors(house, known_factors).iter().filter(|n| **n >= mindiv).sum::<usize>()
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let goal = lines[0].parse::<usize>().unwrap();
    let mut known_factors: HashMap<usize, Vec<usize>> = HashMap::new();
    let factorsum = goal / 10;
    let n = ((goal * 2) as f64 + 0.25).sqrt() - 0.5;
    // find lowest possible number whose factors sum up to factorsum
    for i in n as usize..factorsum {
        let p = presents(i, &mut known_factors);
        if p >= goal {
            return Some(i as i64);
        }
    }
    None
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let goal = lines[0].parse::<usize>().unwrap();
    let mut known_factors: HashMap<usize, Vec<usize>> = HashMap::new();
    let factorsum = goal / 11;
    let n = ((goal * 2) as f64 + 0.25).sqrt() - 0.5;
    for i in n as usize..factorsum {
        let p = presents_part2(i, &mut known_factors, 50);
        if p >= goal {
            return Some(i as i64);
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

    #[test]
    fn test_presents() {
        let mut known_factors = HashMap::new();
        assert_eq!(10, presents(1, &mut known_factors));
        assert_eq!(30, presents(2, &mut known_factors));
        assert_eq!(40, presents(3, &mut known_factors));
        assert_eq!(70, presents(4, &mut known_factors));
        assert_eq!(60, presents(5, &mut known_factors));
        assert_eq!(120, presents(6, &mut known_factors));
        assert_eq!(150, presents(8, &mut known_factors));
        assert_eq!(130, presents(9, &mut known_factors));
        assert_eq!(150 + 50 + 30 + 10, presents(15, &mut known_factors));
    }

    #[test]
    fn test_presents_part2() {
        let mut known_factors = HashMap::new();
        assert_eq!(11, presents_part2(1, &mut known_factors, 50));
        assert_eq!(33, presents_part2(2, &mut known_factors, 50));
        assert_eq!(44, presents_part2(3, &mut known_factors, 50));
        assert_eq!(77, presents_part2(4, &mut known_factors, 50));
        assert_eq!(66, presents_part2(5, &mut known_factors, 50));
        assert_eq!(66 + 33 + 22 + 11, presents_part2(6, &mut known_factors, 50));
    }
}
