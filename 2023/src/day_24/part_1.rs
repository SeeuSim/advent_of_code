/**
 * Inspired by Cranil
 *
 * https://github.com/cranil/aoc2023-rs
 */
use crate::utils::extract_file;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    _z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, _z: z }
    }
}

#[derive(Debug, Clone, Copy)]
struct HailStone {
    position: Point,
    velocity: Point,
}

fn solve_linear(
    (a1, b1, c1): (f64, f64, f64),
    (a2, b2, c2): (f64, f64, f64),
) -> Option<(f64, f64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0.0 {
        None
    } else {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        Some((x, y))
    }
}

impl HailStone {
    fn parse(input: String) -> Option<HailStone> {
        let (_origin, _velocity) = match input.split_once(" @ ") {
            Some(v) => v,
            None => return None,
        };

        let origin = _origin
            .splitn(3, ',')
            .filter_map(|x| x.trim().parse::<i64>().ok())
            .collect::<Vec<_>>();

        let velocity = _velocity
            .splitn(3, ',')
            .filter_map(|x| x.trim().parse::<i64>().ok())
            .collect::<Vec<_>>();

        if origin.len() != 3 || velocity.len() != 3 {
            return None;
        }
        Some(HailStone {
            position: Point {
                x: origin[0],
                y: origin[1],
                _z: origin[2],
            },
            velocity: Point {
                x: velocity[0],
                y: velocity[1],
                _z: velocity[2],
            },
        })
    }

    fn intersection_xy(&self, other: &Self) -> Option<Point> {
        let (x_0, y_0) = (self.position.x, self.position.y);
        let (x_1, y_1) = (other.position.x, other.position.y);
        let (vx_0, vy_0) = (self.velocity.x, self.velocity.y);
        let (vx_1, vy_1) = (other.velocity.x, other.velocity.y);

        let (a0, b0) = (vx_0, -vx_1);
        let c0 = x_1 - x_0;
        let (a1, b1) = (vy_0, -vy_1);
        let c1 = y_1 - y_0;

        if let Some((s, t)) = solve_linear(
            (a0 as f64, b0 as f64, c0 as f64),
            (a1 as f64, b1 as f64, c1 as f64),
        ) {
            if s >= 0.0 && t >= 0.0 {
                let x_x = x_0 as f64 + vx_0 as f64 * s;
                let s_y = y_0 as f64 + vy_0 as f64 * s;
                let t_x = x_1 as f64 + vx_1 as f64 * t;
                let t_y = y_1 as f64 + vy_1 as f64 * t;

                let x = (x_x + t_x) / 2.0;
                let y = (s_y + t_y) / 2.0;

                Some(Point::new(x as i64, y as i64, 0))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn calc_n_intersections(lines: impl Iterator<Item = String>) -> u64 {
    let stones = lines.filter_map(HailStone::parse).collect::<Vec<_>>();

    let count = (0..stones.len())
        .flat_map(|index_1| (index_1 + 1..stones.len()).map(move |index_2| (index_1, index_2)))
        .map(|(index_1, index_2)| (&stones[index_1], &stones[index_2]))
        .filter_map(|(stone1, stone2)| stone1.intersection_xy(stone2))
        .filter(|intersect| {
            let lo = 200000000000000;
            let hi = 400000000000000;
            lo <= intersect.x && hi >= intersect.x && lo <= intersect.y && hi >= intersect.y
        })
        .count();

    count as u64
}

pub fn never_fail_to_tell_me_the_odds(file_name: &String) {
    // let test = r#"
    // 19, 13, 30 @ -2,  1, -2
    // 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @  1, -5, -3
    // "#
    // .trim()
    // .split('\n')
    // .map(|x| x.to_string());

    // let test_ans = calc_n_intersections(test);

    // assert!(test_ans == 2, "Test: {test_ans} should be 2");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader.lines().filter_map(std::result::Result::ok);
    let ans = calc_n_intersections(lines);

    println!("Answer: {ans}");
}
