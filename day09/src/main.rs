#![allow(unused)]
#![allow(dead_code)]

use std::collections::HashMap;

type Node = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Distance {
    dest: Node,
    dist: usize,
}

enum SearchGoal {
    SHORTEST,
    LONGEST
}

impl Distance {
    fn new(dest: Node, dist: usize) -> Distance {
        Distance{dest, dist}
    }
}

#[derive(Debug, Clone, Copy)]
struct DFSFrame {
    node: Node,
    forward: bool,
    dist: usize
}

impl DFSFrame {
    fn new(node: Node, forward: bool, dist: usize) -> DFSFrame {
        DFSFrame{node, forward, dist}
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
    edges: Vec<Vec<Distance>>,
}

impl Graph {
    fn parse(lines: &Vec<&str>) -> Graph {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let mut edges: Vec<Vec<Distance>> = Vec::new();
        let mut num_nodes: Node = 0;

        for line in lines.into_iter() {
            let split: Vec<&str> = line.split(" ").collect();
            assert_eq!(5, split.len());
            for i in [0, 2] {
                if !nodes.contains_key(split[i]) {
                    nodes.insert(String::from(split[i]), num_nodes);
                    num_nodes += 1;
                    edges.push(Vec::new());
                    assert_eq!(edges.len(), nodes.len());
                }
            }
            let node0 = nodes[split[0]];
            let node1 = nodes[split[2]];
            let dist = split[4].parse().expect("Could not parse distance");
            edges[node0].push(Distance::new(node1, dist));
            edges[node1].push(Distance::new(node0, dist));
        }
        Graph{nodes, edges}
    }

    fn get_node(&self, name: &str) -> Node {
        self.nodes[name]
    }

    fn get_edges(&self, name: &str) -> &Vec<Distance> {
        let node = self.get_node(name);
        &self.edges[node]
    }

    fn get_num_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn get_num_edges(&self) -> usize {
        self.edges.iter().map(|e_vec| e_vec.len()).sum::<usize>() / 2
    }

    fn hamiltonian_path(self, goal: SearchGoal) -> usize {
        // DFS
        let mut stack = Vec::new();
        let mut visited: Vec<bool> = Vec::new();
        let mut num_visited = 0;
        let num_nodes = self.get_num_nodes();
        let mut best_path = match(goal) {
            SearchGoal::SHORTEST => usize::MAX,
            SearchGoal::LONGEST => usize::MIN
        };
        visited.resize(num_nodes, false);
        let isbetter = |x, y| {
            match(goal) {
                SearchGoal::SHORTEST => x < y,
                SearchGoal::LONGEST => x > y
            }
        };
        for startnode in (0..self.get_num_nodes()) {
            stack.push(DFSFrame::new(startnode, true, 0));

            while let Some(frame) = stack.pop() {
                // test if we went to full way
                if frame.forward {
                    visited[frame.node] = true;
                    num_visited += 1;
                    // add backtrack frame
                    stack.push(DFSFrame::new(frame.node, false, frame.dist));

                    // test if we got a full hemiltonian path
                    if num_visited == num_nodes && isbetter(frame.dist, best_path) {
                        best_path = frame.dist;
                        continue;
                    }
                    for e in (&self.edges[frame.node]).iter().filter(|e| !visited[e.dest]) {
                        let newdist = frame.dist + e.dist;
                        if isbetter(newdist, best_path) || matches!(goal, SearchGoal::LONGEST) {
                            //push forward frame on stack
                            stack.push(DFSFrame::new(e.dest, true, frame.dist + e.dist));
                        }
                    }
                } else {
                    visited[frame.node] = false;
                    num_visited -= 1;
                }
            }
        }

        best_path
    }
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    Some(Graph::parse(lines).hamiltonian_path(SearchGoal::SHORTEST) as i64)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    Some(Graph::parse(lines).hamiltonian_path(SearchGoal::LONGEST) as i64)
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
    static TESTINPUT: &str = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn test_part1() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(605), part1(&lines));
    }

    #[test]
    fn test_part2() {
        let lines: Vec<&str> = TESTINPUT.lines().collect();
        assert_eq!(Some(982), part2(&lines));
    }
}
