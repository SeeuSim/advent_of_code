use crate::utils::extract_file;
use std::{
    collections::{VecDeque, HashSet},
    io::BufRead,
};

fn bfs_garden(steps: u32, garden: &Vec<Vec<char>>) -> u64 {
    let (n_r, n_c) = (garden.len(), garden[0].len());

    let mut s_pos = (0, 0);
    for r in 0..n_r {
        for c in 0..n_c {
            if matches!(garden[r][c], 'S') {
                s_pos = (r, c);
                break;
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut queue = VecDeque::from([(s_pos, 0)]);

    let mut count = HashSet::new();

    while let Some(((curr_r, curr_c), steps_taken)) = queue.pop_front() {
        if steps_taken == steps || steps_taken % 2 == steps % 2  {
            count.insert((curr_r, curr_c));
        }
        if steps_taken == steps {
            continue;
        }
        for (delta_r, delta_c) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let new_r = curr_r as i32 + delta_r;
            let new_c = curr_c as i32 + delta_c;
            let key = (new_r as usize, new_c as usize);
            if new_r < 0
                || (new_r as usize) >= n_r
                || new_c < 0
                || (new_c as usize) >= n_c
                || visited.contains(&key)
                || matches!(garden[key.0][key.1], '#')
            {
                continue;
            }
            visited.insert(key);
            queue.push_back((key, steps_taken + 1))
        }
    }
    count.len() as u64
}

pub fn step_counter(file_name: &String) {
    let test = r#"
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().chars().collect())
    .collect();

    let test_ans = bfs_garden(6, &test);

    assert!(test_ans == 16, "Test Answer: Expected 16, got {test_ans}");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|x| x.chars().collect())
        .collect();

    let ans = bfs_garden(64, &lines);

    println!("Answer: {ans}");
}
