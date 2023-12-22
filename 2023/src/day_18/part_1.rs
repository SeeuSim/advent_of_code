use crate::utils::extract_file;
use std::{io::BufRead, ops::Div};

fn get_lava_area(instructions: &Vec<String>) -> u32 {
    let instruction_pairs = instructions
        .iter()
        .map(|x| {
            let values = x.trim().split(' ').take(2).collect::<Vec<_>>();
            (
                values[0].to_string(),
                values[1].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut vertices = vec![(0, 0)];
    let circumference = instruction_pairs.iter().fold(0, |acc, (dir, count)| {
        let (delta_y, delta_x): (i32, i32) = match dir.as_str() {
            "U" => (-1, 0),
            "D" => (1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            _ => unreachable!(),
        };
        let last_idx = vertices.len() - 1;
        let (last_y, last_x) = vertices[last_idx];
        vertices.push((
            last_y as i32 + delta_y * *count as i32,
            last_x as i32 + delta_x * *count as i32,
        ));
        acc + *count
    });

    let polygon_area = (0..vertices.len())
        .map(|index| {
            let i_sub1 = match index {
                0 => vertices.len() - 1,
                _ => index - 1
            };

            let i_add1 = match index < vertices.len() - 1{
                true => index + 1,
                false => 0
            };

            vertices[index as usize].0
                * (vertices[i_sub1].1
                    - vertices[i_add1].1)
        })
        .sum::<i32>()
        .abs()
        .div(2);

    let interior = polygon_area - (circumference as i32).div(2) + 1;

    interior as u32 + circumference
}

pub fn lavaduct_lagoon(file_name: &String) {
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

    assert!(test_res == 62, "\n\n==============================\nError: Test Res Obtained: {test_res}\n==============================\n\n");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let instructions = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<_>>();

    let answer = get_lava_area(&instructions);

    println!("Answer: {answer}");
}
