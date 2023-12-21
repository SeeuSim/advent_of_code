use crate::utils::extract_file;
use std::io::BufRead;

fn get_code_score(code: &String) -> u32 {
    code.chars().fold(0_u32, |acc, e| {
        let c_val = e as u32;
        ((acc + c_val) * 17) % 256
    })
}

fn calc_total_refract_score(data: &String) -> u32 {
    let codes = data.split(',').map(|x| x.to_string()).collect::<Vec<_>>();

    let mut lenses: Vec<Vec<(String, u8)>> = vec![vec![]; 256];

    for code in codes {
        let (label, refract_score) = match code.contains("=") {
            true => match code.split_once('=') {
                Some((label, raw_score)) => {
                    (label.to_string(), raw_score.parse::<i8>().ok().unwrap())
                }
                _ => unreachable!(),
            },
            _ => match code.split_once('-') {
                Some(v) => (v.0.to_string(), -1),
                _ => unreachable!(),
            },
        };

        let box_idx = get_code_score(&label) as usize;

        // Remove
        if refract_score >= 0 {
            let pos = lenses[box_idx].iter().position(|x| x.0.eq(&label));
            match pos {
                Some(idx) => {
                    lenses[box_idx][idx].1 = refract_score as u8;
                }
                None => {
                    lenses[box_idx].push((label, refract_score as u8));
                }
            }
        } else {
            let pos = lenses[box_idx].iter().position(|x| x.0.eq(&label));
            if pos.is_some() {
                lenses[box_idx].remove(pos.unwrap());
            }
        }
    }
    lenses
        .iter()
        .enumerate()
        .fold(0_u32, |acc, (box_num, _box)| {
            acc + _box
                .iter()
                .enumerate()
                .map(|(index, (_, foc_len))| {
                    (box_num as u32 + 1) * (index as u32 + 1) * *foc_len as u32
                })
                .sum::<u32>()
        })
}

pub fn lens_library_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let test_data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();

    let test_ans = calc_total_refract_score(&test_data);
    assert!(test_ans == 145, "{test_ans}");

    let data = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .next()
        .unwrap();

    let ans = calc_total_refract_score(&data);

    println!("Answer: {ans}");
}
