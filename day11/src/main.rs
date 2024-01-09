#![allow(unused)]
#![allow(dead_code)]

fn encodepw(pw: &str) -> Vec<u8> {
    pw.bytes().collect()
}

fn decodepw(pw: &[u8]) -> String {
    pw.iter().map(|c| *c as char).collect()
}

static DISALLOWED: [u8; 3] = ['i' as u8, 'o' as u8, 'l' as u8];
fn pwvalid(pw: &[u8]) -> bool {
    let mut streak = 0;
    let mut pairs = [0u8; 2];
    let mut lastchar = 0;
    for c in pw {
        if DISALLOWED.iter().any(|d| d == c) {
            return false;
        }
        if streak < 2 {
            streak = if lastchar + 1 == *c { streak + 1} else { 0 };
        }

        if pairs[1] == 0 && lastchar == *c {
            if pairs[0] == 0 {
                pairs[0] = *c;
            } else if pairs[0] != *c {
                pairs[1] = *c;
            }
        }
        lastchar = *c;
    }
    pairs[1] > 0 && streak >= 2
}

fn iterate(pw: &mut[u8]) {
    const MAXCHAR: u8 = 'z' as u8;
    const MINCHAR: u8 = 'a' as u8;
    // skipahead
    if let(Some(i)) = pw.iter().enumerate().filter(|(i, c)| DISALLOWED.iter().any(|d| d == *c)).map(|(i, c)| i).next() {
        pw[i] += 1;
        for j in (i + 1..pw.len()) {
            pw[j] = MINCHAR;
        }
        return;
    }
    for i in (0..pw.len()).rev() {
        if pw[i] < MAXCHAR {
            pw[i] += 1;
            break;
        } else {
            assert!(i > 0); //if this assertion does not hold, we need flex pw length
            pw[i] = MINCHAR;
        }
    }
}

fn part1(lines: &Vec<&str>) -> Option<String> {
    let mut current = encodepw(lines[0]);
    iterate(&mut current);
    while !pwvalid(&current) {
        iterate(&mut current);
    }
    Some(decodepw(&current))
}

fn part2(lines: &Vec<&str>) -> Option<String> {
    let firstpw = part1(lines).expect("Could not unwrap first pw");
    part1(&firstpw.lines().collect())
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
    static TESTINPUT: &str = "abcdefgh";

    #[test]
    fn test_iterate() {
        let mut pw = encodepw("bzy");
        iterate(&mut pw);
        assert_eq!(&pw, &encodepw("bzz"));
        iterate(&mut pw);
        assert_eq!(&pw, &encodepw("caa"));
        iterate(&mut pw);
        assert_eq!(&pw, &encodepw("cab"));
    }

    #[test]
    fn test_iterate_skipahead() {
        let mut pw = encodepw("hihfgas");
        iterate(&mut pw);
        assert_eq!(decodepw(&pw), "hjaaaaa");
    }

    #[test]
    fn test_encode_decode() {
        let pass = "helloworld";
        assert_eq!(pass, decodepw(&encodepw(pass)));
    }

    #[test]
    fn test_invalid_containsi() {
        assert!(!pwvalid(&encodepw("hijklmmn")));
    }

    #[test]
    fn test_invalid_noincrease() {
        assert!(!pwvalid(&encodepw("abbceffg")));
    }

    #[test]
    fn test_invalid_nosecondpair() {
        assert!(!pwvalid(&encodepw("abbcegjk")));
    }

    #[test]
    fn test_valid() {
        assert!(pwvalid(&encodepw("abcdffaa")));
    }

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(String::from("abcdffaa")), part1(&lines));
    }

    #[test]
    fn test_part1_longskip() {
        let lines: Vec<&str> = "ghijklmn".lines().collect();
        let expected = "ghjaabcc";
        assert!(pwvalid(&encodepw(&expected)));
        assert_eq!(expected, part1(&lines).unwrap().as_str());
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(String::from("abcdffbb")), part2(&lines));
    }
}
