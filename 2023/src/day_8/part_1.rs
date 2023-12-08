//Haunted Wasteland

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

pub fn haunted_wasteland(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let mut it = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .into_iter();

    let instructions = it.next().unwrap().chars().collect::<Vec<char>>();
    let n_instr = instructions.len();

    let graph: HashMap<String, (String, String)> = HashMap::from_iter(
        it.skip(1)
            .filter_map(Node::parse_from_string)
            .map(|node| (node.label, (node.left, node.right))),
    );

    let mut curr = "AAA".to_string();
    let end: String = "ZZZ".to_string();
    let mut step_ct = 0;

    while curr.cmp(&end) != std::cmp::Ordering::Equal {
        let &c_inst = instructions.index(step_ct % n_instr);
        match c_inst {
            'L' => {
                curr = graph.get(&curr).unwrap().0.as_str().to_string();
            }
            'R' => {
                curr = graph.get(&curr).unwrap().1.as_str().to_string();
            }
            _ => {
                break;
            }
        }
        step_ct += 1;
    }
    println!("Number of steps: {}", step_ct);
}
