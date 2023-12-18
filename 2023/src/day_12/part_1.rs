use crate::utils::extract_file;
use std::io::BufRead;

fn process_line(line: Vec<char>, groups: Vec<u8>) -> u32 {
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
    
    let f_c = line[0];
    let f_g = groups[0] as usize;
    let mut res = 0;

    if matches!(f_c, '.' | '?') {
        res += process_line(line[1..].to_vec(), groups.to_vec());
    }

    if matches!(f_c, '#' | '?')
        && f_g <= line.len()
        && !line[..f_g].to_vec().contains(&'.')
        && (f_g == line.len() || line[f_g] != '#')
    {
        let next_line = match f_g + 1 >= line.len() {
            true => vec![],
            _ => line[(f_g + 1)..].to_vec()
        };
        res += process_line(next_line, groups[1..].to_vec())
    }

    res
}

/**
 * Sum all the count of different possible arrangements,
 * given:
 *
 * .#.??????.###. 1, 2, 3
 *
 * In this case, there is 1 contiguous group of size 1 at the start,
 * and 1 contiguous group of size 3 at the back.
 *
 * What's left is to calculate the number of ways you can fit the
 * contiguous group of size 2 in the question marks so that the arrangement
 * matches the count.
 *
 * This has to be generalised to all possible scenarios.
 */
pub fn hot_springs(file_name: &String) {
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

            let groups = match groups.last() {
                Some(v) => v
                    .split(',')
                    .filter_map(|v| v.parse::<u8>().ok())
                    .collect::<Vec<_>>(),
                _ => unreachable!(),
            };

            process_line(line_actual, groups)
        })
    .sum::<u32>();

    println!("Answer: {}", ans);
}
