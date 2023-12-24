use num::Integer;

use crate::utils::extract_file;
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
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
    let n_cycles = 1000;
    let mut module_map: HashMap<&str, Module<'_>> = HashMap::from_iter(
        lines
            .iter()
            .filter_map(|x| Module::parse(x.as_str()))
            .map(|x| (x.label, x)),
    );
    let (_, broadcaster) = module_map.remove_entry("broadcaster").unwrap();
    let initial_targets = broadcaster.outputs;
    let mut n_it = 0;
    let mut acc = vec![];

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

    loop {
        n_it += 1;
        // Origin, target, is_high
        let mut queue =
            VecDeque::from_iter(initial_targets.iter().map(|x| ("broadcaster", *x, false)));
        let (mut lo, mut hi) = (1, 0);
        while !queue.is_empty() {
            let (origin, target, is_high) = match queue.pop_front() {
                Some(v) => v,
                None => break,
            };
            if is_high {
                hi += 1;
            } else {
                lo += 1;
            }

            if !module_map.contains_key(target) {
                continue;
            }

            let module = module_map.get_mut(target).unwrap();

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
        acc.push((lo, hi));

        if n_it >= n_cycles || module_map.values().all(|x| !x.is_on) {
            break;
        }
    }

    let (cycles, excluded) = n_cycles.div_mod_floor(&n_it);
    let prelim_vals = (0..excluded)
        .map(|idx| acc[idx])
        .fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    let vals_per_cycle = acc.iter().fold((0, 0), |acc, e| (acc.0 + e.0, acc.1 + e.1));

    let lo = vals_per_cycle.0 * cycles + prelim_vals.0;
    let hi = vals_per_cycle.1 * cycles + prelim_vals.1;
    lo * hi
}

pub fn pulse_propagation(file_name: &String) {
    let test_1 = r#"
    broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect::<Vec<_>>();

    let test_ans = get_cycles(&test_1);
    assert!(
        test_ans == 32000000,
        "Test Case 1: Expected 32000000, Got {test_ans}"
    );

    let test_2 = r#"
    broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect::<Vec<_>>();

    let test_ans = get_cycles(&test_2);
    assert!(
        test_ans == 11687500,
        "Test Case 2: Expected 11687500, Got {test_ans}"
    );

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let ans = get_cycles(&lines);

    println!("Answer: {ans}");
}
