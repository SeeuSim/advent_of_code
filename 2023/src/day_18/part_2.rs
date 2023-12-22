use crate::utils::extract_file;
use regex::Regex;
use std::{io::BufRead, ops::Div};

fn get_lava_area(instructions: &Vec<String>) -> u64 {
    let re = Regex::new(r#"\(#([a-z0-9]+)\)$"#).unwrap();
    let instruction_pairs = instructions
        .iter()
        .filter_map(|x| re.captures(x.as_str()).unwrap().iter().nth(1).unwrap())
        .map(|v| {
            let values = v.as_str().trim();
            let dir = values.chars().last().unwrap();
            let val = u64::from_str_radix(values.chars().take(5).collect::<String>().as_str(), 16)
                .ok()
                .unwrap();
            (dir, val)
        })
        .collect::<Vec<_>>();

    let mut vertices = vec![(0, 0)];
    let circumference = instruction_pairs.iter().fold(0, |acc, (dir, count)| {
        let (delta_y, delta_x): (i64, i64) = match dir {
            '0' => (0, 1),
            '1' => (1, 0),
            '2' => (0, -1),
            '3' => (-1, 0),
            _ => unreachable!()
        };
        let last_idx = vertices.len() - 1;
        let (last_y, last_x) = vertices[last_idx];
        vertices.push((
            last_y as i64 + delta_y * *count as i64,
            last_x as i64 + delta_x * *count as i64,
        ));
        acc + *count
    });

    let polygon_area = (0..vertices.len())
        .map(|index| {
            let i_sub1 = match index {
                0 => vertices.len() - 1,
                _ => index - 1,
            };

            let i_add1 = match index < vertices.len() - 1 {
                true => index + 1,
                false => 0,
            };

            vertices[index as usize].0 * (vertices[i_sub1].1 - vertices[i_add1].1)
        })
        .sum::<i64>()
        .abs()
        .div(2);

    let interior = polygon_area - (circumference as i64).div(2) + 1;

    interior as u64 + circumference as u64
}

pub fn lavaduct_lagoon_two(file_name: &String) {
    let test_inst = "
    R 6 (#70c710)
    D 5 (#0dc571)
    L 2 (#5713f0)
    D 2 (#d2c081)
    R 2 (#59c680)
    D 2 (#411b91)
    L 5 (#8ceee2)
    U 2 (#caa173)
    L 1 (#1b58a2)
    U 2 (#caa171)
    R 2 (#7807d2)
    U 3 (#a77fa3)
    L 2 (#015232)
    U 2 (#7a21e3)
    "
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect::<Vec<_>>();

    let test_res = get_lava_area(&test_inst);

    assert!(test_res == 952408144115, "\n\n==============================\nError: Test Res Obtained: {test_res}\n==============================\n\n");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let instructions = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let answer = get_lava_area(&instructions);

    println!("Answer: {answer}");
    
}
