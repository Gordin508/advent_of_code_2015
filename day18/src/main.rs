#![allow(unused)]
#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Grid2D {
    data: Vec<i8>,
    width: usize,
    height: usize,
}

impl Grid2D {
    fn new(width: usize, height: usize) -> Grid2D {
        let mut data = Vec::new();
        data.resize(width * height, 0);
        Grid2D {data, width, height}
    }

    fn get(&self, y: isize, x: isize, default: i8) -> i8 {
        if y < 0 || x < 0 || y as usize >= self.width || x as usize >= self.width {
            return default;
        }
        self.data[y as usize * self.width + x as usize]
    }
}

impl From<&Vec<&str>> for Grid2D {
    fn from(lines: &Vec<&str>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let mut grid = Grid2D::new(width, height);
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                grid[y][x] = match char {
                    '#' => 1,
                    _   => 0
                }
            }
        }
        grid
    }
}

use std::ops::{Index, IndexMut};

fn game_of_life(grid: Grid2D, num_steps: usize, freeze_corners: bool) -> usize {
    // what a mess this implementation is...
    assert!(num_steps > 0);
    let mut on = 0;
    let mut grid = grid;
    let width = grid.width;
    let height = grid.height;
    if freeze_corners {
        grid[0][0] = 1;
        grid[height - 1][0] = 1;
        grid[height - 1][width - 1] = 1;
        grid[0][width - 1] = 1;
    }
    let neighborsum = |l_grid: &Grid2D, y: isize, x: isize| -> i8 {
                let mut sum = 0;
                for ny in (y as isize - 1..=y as isize + 1) {
                    for nx in (x as isize - 1..=x as isize + 1) {
                        if ny == y as isize && nx == x as isize {
                            continue;
                        }
                        sum += l_grid.get(ny, nx, 0);
                }
            }
        sum
    };
    for step in (0..num_steps) {
        let mut nextgrid = Grid2D::new(grid.width, grid.height);
        on = 0;
        for y in (0..grid.width) {
            for x in (0..grid.height) {
                let nsum = neighborsum(&grid, y as isize, x as isize);
                if grid[y][x] == 1 && (nsum == 2 || nsum == 3)
                    || grid[y][x] == 0 && nsum == 3
                    || freeze_corners && (y == 0 || y == height - 1) && (x == 0 || x == width - 1) {
                    nextgrid[y][x] = 1;
                    on += 1;
                }
            }
        }
        grid = nextgrid;
    }
    on
}

impl Index<usize> for Grid2D {
    type Output = [i8];
    fn index(&self, row: usize) -> &[i8] {
        let start = row * self.width;
        &self.data[start..start+self.width]
    }
}

impl IndexMut<usize> for Grid2D {
    fn index_mut(&mut self, row: usize) -> &mut [i8] {
        let start = row * self.width;
        &mut self.data[start..start+self.width]
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let grid = Grid2D::from(lines);
    Some(game_of_life(grid, 100, false) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    let grid = Grid2D::from(lines);
    Some(game_of_life(grid, 100, true) as i64)
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
