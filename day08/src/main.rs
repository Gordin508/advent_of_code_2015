#![allow(unused)]
#![allow(dead_code)]


fn get_num_chars_reencode(line: &str) -> usize {
    2 + line.chars().map(|c|
            match c {
                '\\' | '\"' => 2,
                _    => 1
            }
    )
    .sum::<usize>()
}

fn get_num_chars(line: &str) -> usize {
    let mut escaped = false;
    let result = line.chars().map(|c|
        match c {
            '\\' => {
                escaped = !escaped;
                if escaped { 0 } else { 1 }
            },
            'x' => {
                let originalescaped = escaped;
                escaped = false;
                if originalescaped { - 1 } else { 1 } 
            },
            _ => {
                escaped = false;
                1
            }
        
        }
    ).sum::<i64>();
    result as usize - 2
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(lines.iter().map(|line| line.len() - get_num_chars(line)).sum::<usize>() as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(lines.iter().map(|line| get_num_chars_reencode(line) - line.len()).sum::<usize>() as i64)
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

    // static array of test strings
    static TEST_STRINGS: [&str; 4] = ["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];

    #[test]
    fn num_chars_empty() {
        assert_eq!(0, get_num_chars(TEST_STRINGS[0]));
    }

    #[test]
    fn num_chars_simple() {
        assert_eq!(3, get_num_chars(TEST_STRINGS[1]));
    }

    #[test]
    fn num_chars_singleescaped() {
        assert_eq!(7, get_num_chars(TEST_STRINGS[2]));
    }

    #[test]
    fn num_chars_hexcode() {
        assert_eq!(1, get_num_chars(TEST_STRINGS[3]));
    }

    #[test]
    fn num_chars_empty_reencode() {
        assert_eq!(6, get_num_chars_reencode(TEST_STRINGS[0]));
    }

    #[test]
    fn num_chars_simple_reencode() {
        assert_eq!(9, get_num_chars_reencode(TEST_STRINGS[1]));
    }

    #[test]
    fn num_chars_singleescaped_reencode() {
        assert_eq!(16, get_num_chars_reencode(TEST_STRINGS[2]));
    }

    #[test]
    fn num_chars_hexcode_reencode() {
        assert_eq!(11, get_num_chars_reencode(TEST_STRINGS[3]));
    }
    #[test]
    fn num_chars_doubleescape() {
        assert_eq!(6, get_num_chars("\"\\\\hello\""));
    }

    #[test]
    fn num_chars_doubleescape_reencode() {
        assert_eq!(4 + 4 + 5 + 2, get_num_chars_reencode("\"\\\\hello\""));
    }

    #[test]
    fn part1_testinput() {
        let testinput = TEST_STRINGS.to_vec();
        assert_eq!(Some(12), part1(&testinput));
    }

    #[test]
    fn part2_testinput() {
        let testinput = TEST_STRINGS.to_vec();
        assert_eq!(Some(19), part2(&testinput));
    }
}
