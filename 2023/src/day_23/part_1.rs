use crate::utils::extract_file;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn dfs(
    point: (usize, usize),
    end: &(usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    graph: &HashMap<(usize, usize), HashMap<(usize, usize), u32>>,
) -> u32 {
    if point.eq(end) {
        return 0;
    }

    let mut res = 0;
    visited.insert(point);

    for next_pt in graph.get(&point).unwrap().keys() {
        if !visited.contains(next_pt) {
            res = res.max(
                dfs(*next_pt, end, visited, graph)
                    + graph.get(&point).unwrap().get(&next_pt).unwrap(),
            );
        }
    }

    visited.remove(&point);
    res
}

fn get_longest_distance(maze: &Vec<Vec<char>>) -> u32 {
    let (n_r, n_c) = (maze.len(), maze[0].len());

    let start = (0, maze[0].iter().position(|x| *x == '.').unwrap());
    let end = (
        n_r - 1,
        maze[n_r - 1].iter().position(|x| *x == '.').unwrap(),
    );

    // Points of interest
    let mut summarised_graph_pts = vec![start, end];
    for r in 0..n_r {
        for c in 0..n_c {
            if maze[r][c] == '#' {
                continue;
            }
            let neighbours = [(0, -1), (0, 1), (1, 0), (-1, 0)]
                .iter()
                .filter_map(|(d_r, d_c)| {
                    let (f_r, f_c) = ((r as i32) + d_r, (c as i32) + d_c);
                    if 0 > f_r
                        || f_r as usize >= n_r
                        || 0 > f_c
                        || f_c as usize >= n_c
                        || maze[f_r as usize][f_c as usize] == '#'
                    {
                        return None;
                    }
                    Some(())
                })
                .count();
            if neighbours >= 3 {
                summarised_graph_pts.push((r, c));
            }
        }
    }

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), u32>> =
        HashMap::from_iter(summarised_graph_pts.iter().map(|&pt| (pt, HashMap::new())));

    summarised_graph_pts
        .iter()
        .for_each(|&(start_ro, start_col)| {
            let mut stack = vec![(start_ro, start_col, 0)];
            let mut seen = HashSet::from([(start_ro, start_col)]);
            while let Some((r, c, dist)) = stack.pop() {
                if dist > 0 && summarised_graph_pts.contains(&(r, c)) {
                    *graph
                        .get_mut(&(start_ro, start_col))
                        .unwrap()
                        .entry((r, c))
                        .or_insert(0) = dist;
                    continue;
                }
                let dirs = match maze[r][c] {
                    '.' => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
                    '^' => vec![(-1, 0)],
                    'v' => vec![(1, 0)],
                    '<' => vec![(0, -1)],
                    '>' => vec![(0, 1)],
                    _ => vec![],
                };
                for (delta_ro, delta_col) in dirs {
                    let (f_r, f_c) = ((r as i32) + delta_ro, (c as i32) + delta_col);
                    if 0 <= f_r
                        && f_r as usize <= n_r - 1
                        && 0 <= f_c
                        && f_c as usize <= n_c - 1
                        && maze[f_r as usize][f_c as usize] != '#'
                        && !seen.contains(&(f_r as usize, f_c as usize))
                    {
                        stack.push((f_r as usize, f_c as usize, dist + 1));
                        seen.insert((f_r as usize, f_c as usize));
                    }
                }
            }
        });

    let mut seen: HashSet<(usize, usize)> = HashSet::new();

    dfs(start, &end, &mut seen, &graph)
}

pub fn a_long_walk(file_name: &String) {
    let test = r#"
    #.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().chars().collect())
    .collect();

    let test_ans = get_longest_distance(&test);

    assert!(test_ans == 94, "Expected 94, got {test_ans}");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let maze = reader
        .lines()
        .filter_map(|line| match line.ok() {
            Some(v) => Some(v.chars().collect()),
            None => None,
        })
        .collect();
    let ans = get_longest_distance(&maze);
    println!("Answer: {ans}")
}
