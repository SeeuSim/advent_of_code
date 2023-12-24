use crate::utils::extract_file;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Debug)]
struct Rule {
    prop: String,
    value: u64,
    is_less: bool,
    has_cmp: bool,
    destination: String,
}

impl Rule {
    fn parse(input: &str) -> Option<Rule> {
        // Send to dest
        if !input.contains(':') {
            return Some(Rule {
                prop: String::new(),
                value: u64::MIN,
                is_less: false,
                has_cmp: false,
                destination: input.to_string(),
            });
        }
        let mut vals = input.split(':');
        let rule = match vals.next() {
            Some(v) => v,
            None => return None,
        };
        let mut rule_parts = rule.split(|x| matches!(x, '<' | '>'));
        let prop = match rule_parts.next() {
            Some(v) => v.to_string(),
            None => return None,
        };
        let val_opt = match rule_parts.last() {
            Some(v) => v.parse::<u64>(),
            None => return None,
        };
        let value = match val_opt.ok() {
            Some(v) => v,
            None => return None,
        };

        let destination = match vals.last() {
            Some(v) => v.to_string(),
            None => return None,
        };
        Some(Rule {
            prop,
            value,
            is_less: input.contains('<'),
            has_cmp: true,
            destination,
        })
    }
}

#[derive(Debug)]
struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &String) -> Option<Workflow> {
        let workflow_regex = Regex::new(r#"([a-zA-Z]{2,})\{(.*)\}"#).unwrap();
        let captures = match workflow_regex.captures(input) {
            Some(v) => v,
            None => return None,
        };
        let mut values = captures
            .iter()
            .filter_map(|x| x)
            .skip(1)
            .map(|x| x.as_str());

        let label = match values.next() {
            Some(v) => v.to_string(),
            None => return None,
        };
        let rules = match values.next() {
            Some(v) => v.split(',').filter_map(Rule::parse).collect::<Vec<_>>(),
            None => return None,
        };
        Some(Workflow { label, rules })
    }
}

fn count_combinations(
    ranges: HashMap<char, (u64, u64)>,
    workflow_table: &HashMap<String, Vec<Rule>>,
    key: &str,
) -> u64 {
    if key.eq("R") {
        return 0;
    } else if key.eq("A") {
        return ranges
            .values()
            .fold(1, |acc, (lo, hi)| acc * (*hi - *lo + 1));
    }
    let rules = match workflow_table.get(key) {
        None => return 0,
        Some(v) => v,
    };

    let mut total = 0;
    let mut r = ranges.clone();
    for rule in rules {
        if !rule.has_cmp {
            total += count_combinations(r, workflow_table, &rule.destination);
            break;
        }
        let prop = rule.prop.chars().next().unwrap();
        let (low, high) = r.get(&prop).unwrap();
        let (pass, fail) = match rule.is_less {
            true => (
                (*low, *high.min(&(rule.value - 1))),
                (*low.max(&rule.value), *high)
            ),
            false => (
                (*low.max(&(rule.value + 1)), *high),
                (*low, *high.min(&rule.value))
            )
        };

        if pass.0 <= pass.1 {
            let mut ranges_c = r.clone();
            ranges_c.entry(prop).and_modify(|x| {
                x.0 = pass.0;
                x.1 = pass.1;
            });
            total += count_combinations(ranges_c, workflow_table, &rule.destination);
        }

        if fail.0 <= fail.1 {
            r.entry(prop).and_modify(|x| {
                x.0 = fail.0;
                x.1 = fail.1;
            });
        }
    }
    total
}

fn count_accepted_parts_range(lines: &Vec<String>) -> u64 {
    let cl = lines.clone();
    let workflows = cl
        .iter()
        .take_while(|x| x.len() != 0)
        .filter_map(Workflow::parse)
        .map(|x| (x.label, x.rules))
        .collect::<Vec<_>>();

    let table: HashMap<String, Vec<Rule>> = HashMap::from_iter(workflows);

    let start_range = HashMap::from_iter("xmas".chars().map(|x| (x, (1, 4000))));

    count_combinations(start_range, &table, "in")
}

pub fn aplenty_two(file_name: &String) {
    let test = r#"
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}

    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect::<Vec<_>>();

    let test_result = count_accepted_parts_range(&test);

    assert!(
        test_result == 167409079868000,
        "Answer found: {test_result}"
    );

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let ans = count_accepted_parts_range(&lines);

    println!("Answer: {ans}")
}
