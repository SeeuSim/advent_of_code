use crate::utils::extract_file;
use std::{collections::BTreeMap, io::BufRead};

pub fn seed_fertiliser(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let mut line_p = reader.lines().into_iter();

    let seeds = line_p.next();

    let _discard = line_p.next();
    let mut ranges: Vec<BTreeMap<i64, (i64, i64)>> = Vec::new();

    loop {
        let header = line_p.next().unwrap().unwrap();

        let mut curr_range: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
        // Inner Loop
        loop {
            let line = match line_p.next() {
                Some(line_ct) => match line_ct {
                    Ok(line_ct_2) => line_ct_2,
                    Err(e) => {
                        eprintln!("Error: {}", e.to_string());
                        break;
                    }
                },
                None => {
                    break;
                }
            };

            if line.len() == 0 {
                break;
            }
            let mut vals = line.split(" ").map(|x| x.trim().parse::<i64>().unwrap());

            let dest_start = vals.next().unwrap();
            let src_start = vals.next().unwrap();
            let range = vals.last().unwrap();
            //Start, Delta, Stop
            curr_range.insert(src_start, (dest_start - src_start, src_start + range));
        }

        ranges.push(curr_range);

        let dest = header.split(" ").into_iter().next().unwrap();
        if dest.ends_with("location") {
            break;
        }
    }

    let out = seeds
        .unwrap()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|x| x.parse::<i64>().unwrap())
        .map(|seed| {
            let mut out = seed;
            for range_map in ranges.clone() {
                for (src, (delta, end)) in range_map {
                    if out < src {
                        break;
                    } else if out >= src && out <= end {
                        out = out + delta;
                        break;
                    }
                }
            }
            out
        })
        .min()
        .unwrap();
    println!("Result: {}", out);
}
