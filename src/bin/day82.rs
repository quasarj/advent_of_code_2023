/***
This is 82 because there's another copy of 8 currently lost on
my broken PC. But I want to try again using regex to parse
*/
use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;
use std::error::Error;

type NodeMap = HashMap<String, Node>;

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Self {
            left: left.to_string(),
            right: right.to_string(),
        }
    }
}

#[derive(Debug)]
struct Solver {
    start: String,
    current: String,
    directions: String,
    pos: usize,
}

impl Solver {
    fn new(start: &str, directions: &str) -> Self {
        Solver {
            start: start.to_string(),
            current: start.to_string(),
            directions: directions.to_string(),
            pos: 0,
        }
    }

    fn is_solved(&self) -> bool {
        self.current.ends_with("Z")
        //self.current == "ZZZ"
    }

    // make one step and return true if solved
    fn step(&mut self, map: &NodeMap, direction: String) -> bool {
        //println!("Visiting: {}, going {direction}", self.current);

        let node = map.get(&self.current).unwrap();
        match direction.as_str() {
            "L" => self.current = node.left.clone(), // could be faster with references 
            "R" => self.current = node.right.clone(),
            _ => todo!(),
        }

        self.is_solved()
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 8 - Try 2");

    let re = Regex::new(r"(?m)^(...) = \((...), (...)\)$").unwrap(); // apparently it can fail

    let filename = "input/day8.txt";
    //let filename = "input/day8-test3.txt";
    let binding = read_to_string(filename).unwrap();
    let first_line = binding.lines().next().unwrap();


    let mut nodemap: HashMap<String, Node> = HashMap::new();

    for (_, [name, left, right]) in re.captures_iter(&binding).map(|c| c.extract()) {
        let n = Node::new(left, right);
        nodemap.insert(name.to_string(), n);
    }

    //println!("{:?}", nodemap);
    let mut solvers: Vec<Solver> = vec![];
    // find all "starting nodes" (nodes that end with A)
    for n in nodemap.keys() {
        if n.ends_with("A") {
            println!("{n}");
            solvers.push(Solver::new(n, &first_line));
        }
    }

    // traverse ?
    let directions: Vec<_> = first_line.chars().map(|x| x.to_string()).collect();
    let repeat = directions.iter().cloned().cycle();

    let mut steps = 0;
    for i in repeat {
        steps += 1;
        for s in &mut solvers {
            s.step(&nodemap, i.clone());
        }

        // are they all solved?
        if solvers.iter().all(|x| x.is_solved()) {
            break;
        }

        //if s.step(&nodemap, i) {
        //    break;
        //}


        //if steps > 10000000 {
        ////if steps > 3 {
        //    panic!("giving up");
        //}
    }
    println!("{steps}");

    Ok(())

}
