use crate::utils::extract_file;
use num::integer::gcd;
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
    ops::Div,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

#[derive(Clone, Debug)]
struct Module<'a> {
    mod_type: ModuleType,
    label: &'a str,
    outputs: Vec<&'a str>,
    inputs: HashMap<&'a str, bool>,
    is_on: bool,
}

impl Module<'static> {
    fn parse(input: &str) -> Option<Module> {
        let mut values = input.split(" -> ");
        let mut mod_type = ModuleType::Broadcaster;
        let label = match values.next() {
            Some(v) => match v.as_bytes()[0] {
                b'%' => {
                    mod_type = ModuleType::FlipFlop;
                    &v[1..]
                }
                b'&' => {
                    mod_type = ModuleType::Conjunction;
                    &v[1..]
                }
                _ => v,
            },
            None => return None,
        };

        let outputs = match values.next() {
            Some(v) => v.split(", ").collect::<Vec<_>>(),
            None => return None,
        };

        Some(Module {
            mod_type,
            label,
            outputs,
            inputs: HashMap::new(),
            is_on: false,
        })
    }
}

fn get_cycles(lines: &Vec<String>) -> usize {
    let mut rx_parent = "";
    let mut module_map: HashMap<&str, Module<'_>> = HashMap::from_iter(
        lines
            .iter()
            .filter_map(|x| Module::parse(x.as_str()))
            .map(|x| {
                if x.outputs.contains(&"rx") {
                    rx_parent = x.label;
                }
                (x.label, x)
            }),
    );
    let (_, broadcaster) = module_map.remove_entry("broadcaster").unwrap();
    let initial_targets = broadcaster.outputs;
    let mut n_it = 0;

    // TODO: Try avoiding clone to avoid mutable and immutable borrow mix.
    // Initialise conjunction memory
    for v in module_map
        .clone()
        .values()
        .filter(|x| x.mod_type == ModuleType::FlipFlop)
    {
        for label in v.outputs.iter() {
            let child = module_map.get_mut(*label).unwrap();
            if child.mod_type != ModuleType::Conjunction {
                continue;
            }
            child.inputs.entry(v.label).or_insert(false);
        }
    }

    // For each k-v pair, after v cycles, the module with label k
    // will send a high pulse to the node sending low pulses to rx.
    let mut cycle_lengths: HashMap<&str, usize> = HashMap::new();
    // All the nodes which send high pulses to the node which feeds rx.
    let mut rx_parents_inputs: HashMap<&str, u32> = HashMap::from_iter(
        module_map
            .iter()
            .filter(|x| x.1.outputs.contains(&rx_parent))
            .map(|x| (x.1.label, 0)),
    );

    loop {
        n_it += 1;
        // Origin, target, is_high
        let mut queue =
            VecDeque::from_iter(initial_targets.iter().map(|x| ("broadcaster", *x, false)));
        while !queue.is_empty() {
            let (origin, target, is_high) = match queue.pop_front() {
                Some(v) => v,
                None => break,
            };

            if !module_map.contains_key(target) {
                continue;
            }

            let module = module_map.get_mut(target).unwrap();

            if module.label == rx_parent && is_high {
                *rx_parents_inputs.entry(origin).or_insert(0) += 1;

                if !cycle_lengths.contains_key(origin) {
                    cycle_lengths.insert(origin, n_it);
                }

                // All required nodes have been hit.
                if rx_parents_inputs.values().all(|x| *x > 0) {
                    // The lcm of iterations for all nodes required to send
                    // a high pulse to the node sending a low pulse to rx.
                    return cycle_lengths
                        .values()
                        .fold(1, |acc, e| (acc * *e).div(gcd(*e, acc)));
                }
            }

            match module.mod_type {
                ModuleType::FlipFlop => {
                    if is_high {
                        continue;
                    }
                    module.is_on = !(module.is_on);
                    let is_out_high = module.is_on;
                    module
                        .outputs
                        .iter()
                        .for_each(|x| queue.push_back((module.label, *x, is_out_high)));
                }
                ModuleType::Conjunction => {
                    let in_state = module.inputs.entry(origin).or_insert(false);
                    *in_state = is_high;

                    let is_out_high = !module.inputs.values().all(|x| *x);
                    module
                        .outputs
                        .iter()
                        .for_each(|x| queue.push_back((module.label, *x, is_out_high)));
                }
                _ => continue,
            }
        }
        if module_map.values().all(|x| !x.is_on) {
            break;
        }
    }
    n_it
}

pub fn pulse_propagation_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let ans = get_cycles(&lines);

    println!("Answer: {ans}");
}
