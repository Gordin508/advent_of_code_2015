#![allow(unused)]
#![allow(dead_code)]

use std::{hash::{Hash, Hasher}, borrow::BorrowMut, ops::DerefMut};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

trait Evaluate {
    fn evaluate(&mut self) -> u16;
}

#[derive(Debug, Clone)]
enum Operator {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    CONST(u16),
    PASSTHROUGH
}

impl Operator {
    fn new(op: &str) -> Operator {
        match op {
            "AND" => Operator::AND,
            "OR" => Operator::OR,
            "LSHIFT" => Operator::LSHIFT,
            "RSHIFT" => Operator::RSHIFT,
            "NOT" => Operator::NOT,
            _ => panic!("Unknown operator {}", op)
        }
    }
    fn new_const(value: u16) -> Operator {
        Operator::CONST(value)
    }
}

// Wire has two inputs: left and right
// both have to implement Evaluate
// left and right are either None, or a smartpointer to another Wire
#[derive(Debug, Clone)]
struct Wire {
    name: String,
    left: Option<Rc<RefCell<Wire>>>,
    right: Option<Rc<RefCell<Wire>>>,
    operator: Operator,
    value: Option<u16>
}

impl Hash for Wire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Wire {
    fn new(name: &str, operator: Operator) -> Wire {
        Wire{name: String::from(name), left: None, right: None, operator, value: None}
    }

    fn new_const(identifier: &str) -> Option<Wire> {
        match identifier.parse() {
            Ok(value) => Some(Wire{name: String::new(), operator: Operator::CONST(value), left: None, right: None, value: None}),
            Err(_) => None
        }
    }
    
    fn new_const_u16(value: u16) -> Wire {
        Wire{name: String::new(), operator: Operator::CONST(value), left: None, right: None, value: None}
    }
}

impl Evaluate for Wire {
    // evaluate the wire and cache the result in wire (needs to be mutable)
    fn evaluate(&mut self) -> u16 {
        // evaluate the wire, make sure that left is mutable
        if let Some(result) = self.value {
            return result;
        };
        let lefteval = match self.left {
            Some(ref left) => left.as_ref().borrow_mut().evaluate(),
            None => 0u16
        };
        let righteval = match self.right {
            Some(ref right) => right.as_ref().borrow_mut().evaluate(),
            None => 0u16
        };
        let result = match self.operator {
            Operator::CONST(value) => value,
            Operator::AND => lefteval & righteval,
            Operator::OR => lefteval | righteval,
            Operator::LSHIFT => lefteval << righteval,
            Operator::RSHIFT => lefteval >> righteval,
            Operator::NOT => !lefteval,
            Operator::PASSTHROUGH => lefteval
        };
        self.value = Some(result);
        result
    }
}

fn buildgraph(lines: &Vec<&str>) -> HashMap<String, Rc<RefCell<Wire>>> {
    let mut wires: HashMap<String, Rc<RefCell<Wire>>> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let wire_name = parts[1];
        let inputs: Vec<&str> = parts[0].split(" ").collect();
        match inputs.len() {
            1 => {
                let left = match(Wire::new_const(inputs[0])) {
                    Some(wire) => Some(Rc::new(RefCell::new(wire))),
                    None => None
                };
                let mut gate = Wire::new(wire_name, Operator::PASSTHROUGH);
                gate.left = left;
                wires.insert(String::from(wire_name), Rc::new(RefCell::new(gate)));
            },
            2 => {
                let left = match(Wire::new_const(inputs[1])) {
                    Some(wire) => Some(Rc::new(RefCell::new(wire))),
                    None => None
                };
                let mut gate = Wire::new(wire_name, Operator::new(inputs[0]));
                gate.left = left;
                wires.insert(String::from(wire_name), Rc::new(RefCell::new(gate)));
            },
            3 => {
                let left = match(Wire::new_const(inputs[0])) {
                    Some(wire) => Some(Rc::new(RefCell::new(wire))),
                    None => None
                };
                let right = match(Wire::new_const(inputs[2])) {
                    Some(wire) => Some(Rc::new(RefCell::new(wire))),
                    None => None
                };
                let mut gate = Wire::new(wire_name, Operator::new(inputs[1]));
                gate.left = left;
                gate.right = right;
                wires.insert(String::from(wire_name), Rc::new(RefCell::new(gate)));
            }
            _ => panic!("Invalid input")
        }
    }

    // due to our nasty buildup, we still need to actually connect the wires (painful)
    for line in lines {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let wire_name = parts[1];
        let inputs: Vec<&str> = parts[0].split(" ").collect();
        let mut thiswire: Rc<RefCell<Wire>> = wires.get_mut(wire_name).unwrap().clone();
        match inputs.len() {
            1 => {
                // only necessary if inputs[0] is not a number, for this check if all characters are digits
                if !inputs[0].chars().all(char::is_numeric) {
                    let left = wires.get(inputs[0]).unwrap().clone();
                    thiswire.as_ref().borrow_mut().left = Some(left);
                }
            },
            2 => {
                if let Some(left) = wires.get(inputs[1]) {
                    thiswire.as_ref().borrow_mut().left = Some(left.clone());
                }
            },
            3 => {
                if let Some(left) = wires.get(inputs[0]) {
                    thiswire.as_ref().borrow_mut().left = Some(left.clone());
                };
                if let Some(right) = wires.get(inputs[2]) {
                    thiswire.as_ref().borrow_mut().right = Some(right.clone());
                };
            }
            _ => panic!("Invalid input")
        }
    }
    wires
}

fn part1(lines: &Vec<&str>) -> Option<i64> {
    let wires = buildgraph(lines);
    let endnode = wires.get("a").unwrap();
    let result = endnode.as_ref().borrow_mut().evaluate() as i64;
    Some(result)
}

fn part2(lines: &Vec<&str>) -> Option<i64> {
    // Since we allow running the parts independendly, we reinvoke part1
    let part1result = part1(lines).unwrap() as u16;
    let wires = buildgraph(lines);
    let b = wires.get("b").unwrap();
    b.as_ref().borrow_mut().left = Some(Rc::new(RefCell::new(Wire::new_const_u16(part1result))));
    let endnode = wires.get("a").unwrap();
    let result = endnode.as_ref().borrow_mut().evaluate() as i64;
    Some(result)
}


use std::time::Duration;
struct RunResult {
    value: Option<i64>,
    elapsed: Duration
}

fn main() {
    use std::fs;
    use std::env;
    use std::time::Instant;
    use std::thread;
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
    let mut results = Vec::new();
    thread::scope(|scope| {
        let mut handles = Vec::new();
        for (index, part) in parts.iter().enumerate() {
            handles.push(scope.spawn(|| {
                let partstart = Instant::now();
                let value = part(&lines);
                RunResult{value, elapsed: partstart.elapsed()}
            }));
        }
        for handle in handles {
            results.push(handle.join().unwrap());
        }
    });
    for (index, runresult) in results.into_iter().enumerate() {
        match runresult.value {
            Some(result) => println!("Part {}: {}\t({:?})", index+1, result, runresult.elapsed),
            None => println!("Part {}: No result", index+1),
        }
    }
}
