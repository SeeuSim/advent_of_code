use crate::utils::extract_file;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, graph::UnGraph};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use std::{collections::HashMap, io::BufRead};

fn get_ans(it: impl Iterator<Item = String>) -> u32 {
    let mut graph = UnGraph::<String, u32>::default();
    let nodes = it
        .map(|line| {
            let v = line.split(": ").collect::<Vec<_>>();
            (v[0].to_string(), v[1].to_string())
        })
        .map(|(node, rest)| {
            (
                node,
                rest.split(' ')
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let uniques = nodes
        .iter()
        .flat_map(|(node, rest)| {
            let mut v = rest.clone();
            v.push(node.to_string());
            v
        })
        .unique()
        .collect::<Vec<_>>();

    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    uniques.iter().for_each(|x| {
        let index = graph.add_node(x.clone());
        node_map.insert(x.clone(), index);
    });

    nodes.iter().for_each(|(node, rest)| {
        rest.iter().for_each(|other| {
            graph.add_edge(
                *node_map.get(node).unwrap(),
                *node_map.get(other).unwrap(),
                1,
            );
        });
    });

    let min: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (_, nodes_in_partition) = min.unwrap().unwrap();
    let total_nodes = uniques.len();

    ((total_nodes - nodes_in_partition.len()) * nodes_in_partition.len()) as u32
}

pub fn snowverload(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let lines = reader.lines().filter_map(std::result::Result::ok);

    let ans = get_ans(lines);

    println!("Answer: {ans}");
}
