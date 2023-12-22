use crate::utils::extract_file;
use std::io::BufRead;

fn get_code_score(code: &String) -> u32 {
    code.chars().fold(0_u32, |acc, e| {
        let c_val = e as u32;
        ((acc + c_val) * 17) % 256
    })
}

fn get_score(data: &String) -> u32 {
    let codes = data.split(',').map(|x| x.to_string()).collect::<Vec<_>>();
    codes.iter().map(|x| get_code_score(x)).sum()
}

pub fn lens_library(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    assert!(get_code_score(&"HASH".to_string()) == 52);

    let data = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .next()
        .unwrap();

    let ans = get_score(&data);

    println!("Answer: {ans}");
}
