use crate::utils::extract_file;
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
    ops::Div,
};

fn count_cells_reachable(steps: u64, garden: &Vec<Vec<char>>, (s_r, s_c): (usize, usize)) -> u64 {
    let (n_r, n_c) = (garden.len(), garden[0].len());
    let mut queue = VecDeque::from([((s_r, s_c), steps)]);

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut count: HashSet<(usize, usize)> = HashSet::new();
    while let Some(((curr_ro, curr_col), steps_taken)) = queue.pop_front() {
        // If steps is odd, count every other
        // Else, count incl start point (even)
        if steps_taken % 2 == 0 {
            count.insert((curr_ro, curr_col));
        }
        if steps_taken == 0 {
            continue;
        }
        for (delta_r, delta_c) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let new_r = curr_ro as i32 + delta_r;
            let new_c = curr_col as i32 + delta_c;
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
            queue.push_back((key, steps_taken - 1))
        }
    }
    count.len() as u64
}

fn infinite_bfs_garden(steps: u64, garden: &Vec<Vec<char>>) -> u64 {
    // For our input, it is 131 by 131.
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

    // Number of grids that can be explored in either direction
    // of the starting grid, like this:
    // | g_3 | g_2 | g_1 | S | g_1 | g_2 | g_3 |
    let large_grid_width = steps.div(n_c as u64) - 1;

    /*
        Number of grids external **including** start grid that are odd num of steps away
        Since input grid size is 131, and we start in the center,
        Edge of these grids are:
        65 + 2j * 131 => odd steps away, for some j

        | g_2 |    | S |     | g_2 |
    */
    let odd_grids = (large_grid_width / 2 * 2 + 1).pow(2);
    /*
       Number of grids external **excluding** start grid that are even num of steps away
       Since input grid size is 131, and we start in the center,
       Edge of these grids are:
       65 + 131 + 2k * 131 => even steps away, for some k

       | g_1 | S | g_1 |

    */
    let even_grids = ((large_grid_width + 1) / 2 * 2).pow(2);

    let square_len = n_c as u64;

    let odd_points = count_cells_reachable(square_len * 2 + 1, garden, s_pos);
    let even_points = count_cells_reachable(square_len * 2, garden, s_pos);

    let corner_houses = [
        (n_c - 1, s_pos.1), // Top corner, from Bottom Edge
        (s_pos.0, 0),       // Right corner, from Left Edge
        (0, s_pos.1),       // Bottom Corner, from Top Edge
        (s_pos.0, n_c - 1), // Left Corner, from Right Edge
    ]
    .iter()
    .map(|s_coords| count_cells_reachable(square_len - 1, garden, *s_coords))
    .sum::<u64>();

    let small_triangles = [
        (n_c - 1, 0),       // Top Right Edge, from bottom left
        (n_c - 1, n_c - 1), // Top Left Edge, from bottom right
        (0, 0),             // Bottom Right Edge, from top left
        (0, n_c - 1),       // Bottom Left Edge, from top right
    ]
    .iter()
    .map(|s_coords| count_cells_reachable(square_len / 2 - 1, garden, *s_coords))
    .sum::<u64>();

    let big_triangles = [
        (n_c - 1, 0),       // Top Right Edge, from bottom left
        (n_c - 1, n_c - 1), // Top Left Edge, from bottom right
        (0, 0),             // Bottom Right Edge, from top left
        (0, n_c - 1),       // Bottom Left Edge, from top right
    ]
    .iter()
    .map(|s_coords| count_cells_reachable(square_len * 3 / 2 - 1, garden, *s_coords))
    .sum::<u64>();

    (odd_grids * odd_points)
        + (even_grids * even_points)
        + corner_houses
        + (large_grid_width + 1) * small_triangles
        + (large_grid_width * big_triangles)
}

pub fn placeholder_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let lines = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|x| x.chars().collect())
        .collect();

    let ans = infinite_bfs_garden(26_501_365, &lines);

    println!("Answer: {ans}");
}
