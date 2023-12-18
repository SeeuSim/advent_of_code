use crate::utils::extract_file;
use std::{collections::HashMap, io::BufRead};

fn process_line(
    line: Vec<char>,
    groups: Vec<u8>,
    cache: &mut HashMap<(String, String), u64>,
) -> u64 {
    if line.len() == 0 {
        if groups.len() == 0 {
            return 1;
        }
        return 0;
    }

    if groups.len() == 0 {
        if line.contains(&'#') {
            return 0;
        }
        return 1;
    }

    let k_l = line
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let k_g = groups
        .clone()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let key = (k_l, k_g);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let f_c = line[0];
    let f_g = groups[0] as usize;
    let mut res = 0;

    if matches!(f_c, '.' | '?') {
        res += process_line(line[1..].to_vec(), groups.to_vec(), cache);
    }

    if matches!(f_c, '#' | '?')
        && f_g <= line.len()
        && !line[..f_g].to_vec().contains(&'.')
        && (f_g == line.len() || line[f_g] != '#')
    {
        let next_line = match f_g + 1 >= line.len() {
            true => vec![],
            _ => line[(f_g + 1)..].to_vec(),
        };
        res += process_line(next_line, groups[1..].to_vec(), cache)
    }
    cache.insert(key, res);
    res
}

pub fn hot_springs_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let ans = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|_line| {
            let mut groups = _line.split_whitespace();

            let line_actual = match groups.next() {
                Some(v) => v.chars().collect::<Vec<_>>(),
                _ => unreachable!(),
            };

            let mut repeated_line = vec![];

            for i in 0..5 {
                repeated_line.append(&mut line_actual.clone());
                if i != 4 {
                    repeated_line.push('?');
                }
            }

            let groups = match groups.last() {
                Some(v) => v
                    .split(',')
                    .filter_map(|v| v.parse::<u8>().ok())
                    .collect::<Vec<_>>()
                    .repeat(5),
                _ => unreachable!(),
            };

            let mut cache = HashMap::new();
            process_line(repeated_line, groups, &mut cache)
        })
        .sum::<u64>();

    println!("Answer: {}", ans);
}
