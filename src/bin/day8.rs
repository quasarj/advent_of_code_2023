use std::collections::HashMap;
// use core::cmp::Ordering;
use std::fs::read_to_string;
use std::str::FromStr;
// use regex::Regex;

#[derive(Debug, Clone)]
struct NodeParseError;

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(name: &str, left: &str, right: &str) -> Self {
        Self {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }
    }

    fn from_str(s: &str) -> Self {
        // could probably be a lot better as a regex..
        let parts: Vec<_> = s.split(" = ").collect();
        let name = parts.first().unwrap();
        let inside = parts.last().unwrap();

        let stripped_inside = inside.replace("(", "").replace(")", "");
        let inside_parts: Vec<_> = stripped_inside.split(", ").collect();

        let left = inside_parts.first().unwrap();
        let right = inside_parts.last().unwrap();

        Self::new(name, left, right)
    }
}

impl FromStr for Node {
    type Err = NodeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Node::from_str(s))
    }
}

fn main() {
    println!("Day 8");

    // AAA = (BBB, CCC)
    // BBB = (DDD, EEE)
    // CCC = (ZZZ, GGG)
    // DDD = (DDD, DDD)
    // EEE = (EEE, EEE)
    // GGG = (GGG, GGG)
    // ZZZ = (ZZZ, ZZZ)

    // let n1 = Node::new("AAA", "BBB", "CCC");
    // println!("{:?}", n1);

    // if let Ok(n2) = "abc".parse::<Node>() {
    //     println!("{:?}", n2);
    // }

    // let n3 = Node::from_str("AAA = (BBB, CCC)");

    let filename = "input/day8-test.txt";
    let input_file = read_to_string(filename).unwrap();
    let mut lines = input_file.lines();

    let fl = lines.next().expect("there must be a first line!");
    let blank = lines.next();

    println!("{}", fl);

    let mut nodemap: HashMap<String, Node> = HashMap::new();

    for l in lines {
        if let Ok(node) = l.parse::<Node>() {
            nodemap.insert(node.name.clone(), node);
            // println!("{:?}", node);
            // println!("{}", node.name);
        }
    }

    println!("{:?}", nodemap);

}
