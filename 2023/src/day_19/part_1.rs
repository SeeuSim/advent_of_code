use crate::utils::extract_file;
use regex::Regex;
use std::{collections::HashMap, io::BufRead};

#[derive(Clone, Copy, Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(input: &String) -> Option<Part> {
        let extract_val_regex = Regex::new(r#"\{(.*)\}"#).unwrap();
        let values = match extract_val_regex.captures(input) {
            Some(captures) => captures,
            None => return None,
        };

        let extract_prop_regex = Regex::new(r#"([xmas])=(\d+)"#).unwrap();
        let raw_vals = match values.iter().nth(1).unwrap() {
            None => return None,
            Some(val) => val.as_str().split(','),
        };

        let mut out = Part {
            x: u32::MIN,
            m: u32::MIN,
            a: u32::MIN,
            s: u32::MIN,
        };

        raw_vals.for_each(|v| {
            let _iter = extract_prop_regex.captures(v).unwrap();
            let mut labels_value = _iter.iter().skip(1);

            let label = labels_value.next().unwrap().unwrap().as_str();
            let value = labels_value
                .next()
                .unwrap()
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            match label {
                "x" => out.x = value,
                "m" => out.m = value,
                "a" => out.a = value,
                "s" => out.s = value,
                _ => unreachable!(),
            };
        });
        Some(out)
    }
}

struct ProcResult {
    destination: String,
    go_next: bool,
}

#[derive(Clone, Debug)]
struct Rule {
    prop: String,
    value: u32,
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
                value: u32::MIN,
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
            Some(v) => v.parse::<u32>(),
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

    fn process(&self, part: &Part) -> ProcResult {
        if !self.has_cmp {
            return ProcResult {
                destination: self.destination.clone(),
                go_next: false,
            };
        }

        let p = match self.prop.as_str() {
            "x" => part.x,
            "m" => part.m,
            "a" => part.a,
            "s" => part.s,
            _ => unreachable!(),
        };
        if (self.is_less && p < self.value) || (!self.is_less && p > self.value) {
            return ProcResult {
                destination: self.destination.clone(),
                go_next: false,
            };
        }
        ProcResult {
            go_next: true,
            destination: String::new(),
        }
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

fn count_accepted_parts(lines: &Vec<String>) -> u32 {
    let cl = lines.clone();
    let workflows = cl
        .iter()
        .take_while(|x| x.len() != 0)
        .filter_map(Workflow::parse)
        .map(|x| (x.label, x.rules))
        .collect::<Vec<_>>();

    let parts = lines
        .iter()
        .skip(workflows.len() + 1)
        .filter_map(Part::parse);

    let table: HashMap<String, Vec<Rule>> = HashMap::from_iter(workflows);

    parts
        .map(|p| {
            let mut label = String::from("in");
            while label.ne(&String::from("R")) || label.ne(&String::from("A")) {
                let rules = match table.get(&label) {
                    Some(v) => v,
                    None => {
                        match label.as_str() {
                            "R" => return 0,
                            "A" => return p.x + p.m + p.a + p.s,
                            _ => unreachable!()
                        }
                    }
                };
                for rule in rules {
                    let result = rule.process(&p);
                    if !result.go_next {
                        label = result.destination.clone();
                        break;
                    }
                }
            }
            match label.as_str() {
                "R" => 0,
                "A" => p.x + p.m + p.a + p.s,
                _ => unreachable!(),
            }
        })
        .sum::<u32>()
}
pub fn aplenty(file_name: &String) {
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

    let test_result = count_accepted_parts(&test);

    assert!(test_result == 19114, "Answer found: {test_result}");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let ans = count_accepted_parts(&lines);

    println!("Answer: {ans}")
}
