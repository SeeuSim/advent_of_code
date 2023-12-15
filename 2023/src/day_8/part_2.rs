//Haunted Wasteland 2

use crate::utils::extract_file;
use std::{collections::HashMap, io::BufRead, ops::Index};

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn parse_from_string(line: String) -> Option<Node> {
        let mut label_next = line.split(" = (").into_iter();
        let label = match label_next.next() {
            Some(l) => l.to_string(),
            None => {
                return None;
            }
        };

        let next_nodes_strings = match label_next.next() {
            Some(v) => v.to_string(),
            None => {
                return None;
            }
        };

        let next_nodes = next_nodes_strings.replace(")", "");
        let mut next_nodes = next_nodes.trim().split(", ");

        let left = match next_nodes.next() {
            Some(v) => v.to_string(),
            None => {
                return None;
            }
        };

        let right = match next_nodes.next() {
            Some(v) => v.to_string(),
            None => {
                return None;
            }
        };

        Some(Node { label, left, right })
    }
}

pub fn haunted_wasteland_part_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let mut it = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .into_iter();

    let instructions = it.next().unwrap().chars().collect::<Vec<char>>();
    let n_instr = instructions.len();

    let mut curr_nodes: Vec<String> = vec![];

    let graph: HashMap<String, (String, String)> =
        HashMap::from_iter(it.skip(1).filter_map(Node::parse_from_string).map(|node| {
            if node.label.to_string().ends_with("A") {
                curr_nodes.push(node.label.to_string());
            }
            (node.label, (node.left, node.right))
        }));

    let n_steps = curr_nodes
        .into_iter()
        .map(|node| {
            let mut curr = node.to_string();
            let mut step_ct = 0;

            while !curr.ends_with("Z") {
                let &c_inst = instructions.index((step_ct % n_instr as u64) as usize);
                match c_inst {
                    'L' => {
                        curr = graph.get(&curr).unwrap().0.to_string();
                    }
                    'R' => {
                        curr = graph.get(&curr).unwrap().1.to_string();
                    }
                    _ => unreachable!(),
                };
                step_ct += 1;
            }
            step_ct
        })
        // After wrapping around the instructions this many times,
        // all will land on nodes ending w 'Z'.
        .fold(1, num::integer::lcm);
    println!("Steps: {}", n_steps);
}
