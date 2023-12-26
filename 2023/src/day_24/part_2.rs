/**
 * Inspired by Cranil
 *
 * https://github.com/cranil/aoc2023-rs
 */

use crate::utils::extract_file;
use itertools::Itertools;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
struct HailStone {
    position: Point,
    velocity: Point,
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
                z: origin[2],
            },
            velocity: Point {
                x: velocity[0],
                y: velocity[1],
                z: velocity[2],
            },
        })
    }
}

fn gauss_elimination(matrix: &mut [[f64; 7]; 6]) {
    let n = matrix.len();
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if matrix[j][i].abs() > matrix[max_row][i].abs() {
                max_row = j;
            }
        }

        for k in i..n + 1 {
            let tmp = matrix[max_row][k];
            matrix[max_row][k] = matrix[i][k];
            matrix[i][k] = tmp;
        }

        for j in i + 1..n {
            let c = matrix[j][i] / matrix[i][i];
            for k in i..n + 1 {
                if i == k {
                    matrix[j][k] = 0.0;
                } else {
                    matrix[j][k] -= c * matrix[i][k];
                }
            }
        }
    }

    for i in (0..n).rev() {
        matrix[i][n] /= matrix[i][i];
        matrix[i][i] = 1.0;
        for j in 0..i {
            matrix[j][n] -= matrix[j][i] * matrix[i][n];
            matrix[j][i] = 0.0;
        }
    }
}

fn cross(a: &Point) -> [[i64; 3]; 3] {
    [[0, -a.z, a.y], [a.z, 0, -a.x], [-a.y, a.x, 0]]
}

fn matvecmul(matrix: [[i64; 3]; 3], b: [i64; 3]) -> [i64; 3] {
    let mut c = [0; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i] += matrix[i][j] * b[j];
        }
    }
    c
}

fn calc_intersection_pt(lines: impl Iterator<Item = String>) -> i64 {
    let stones = lines.filter_map(HailStone::parse).collect::<Vec<_>>();

    let p0 = stones[0].position;
    let p1 = stones[1].position;
    let p2 = stones[2].position;

    let v0 = stones[0].velocity;
    let v1 = stones[1].velocity;
    let v2 = stones[2].velocity;

    let delta_pos_0 = Point::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
    let delta_pos_1 = Point::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);

    let delta_vel_0 = Point::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
    let delta_vel_1 = Point::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);

    let cross_posx_0 = cross(&delta_pos_0);
    let cross_posx_1 = cross(&delta_pos_1);

    let cross_velx_0 = cross(&delta_vel_0);
    let cross_velx_1 = cross(&delta_vel_1);

    // M = [[-cross_velx_0, cross_posx_0], [-cross_velx_1, cross_posx_1]]
    let mut coeffs = [[0.0; 7]; 6];
    for i in 0..3 {
        for j in 0..3 {
            coeffs[i][j] = -cross_velx_0[i][j] as f64;
            coeffs[i][j + 3] = cross_posx_0[i][j] as f64;
            coeffs[i + 3][j] = -cross_velx_1[i][j] as f64;
            coeffs[i + 3][j + 3] = cross_posx_1[i][j] as f64;
        }
    }

    // B = [p1xv1 - p0xv0, p2xv2 - p0xv0]
    let p0x = cross(&p0);
    let p1x = cross(&p1);
    let p2x = cross(&p2);

    let p0xv0 = matvecmul(p0x, [v0.x, v0.y, v0.z]);
    let p1xv1 = matvecmul(p1x, [v1.x, v1.y, v1.z]);
    let p2xv2 = matvecmul(p2x, [v2.x, v2.y, v2.z]);

    for i in 0..3 {
        coeffs[i][6] = (p1xv1[i] - p0xv0[i]) as f64;
        coeffs[i + 3][6] = (p2xv2[i] - p1xv1[i]) as f64;
    }
    
    gauss_elimination(&mut coeffs);

    let rock_pos = Point::new(
        coeffs[0][6] as i64,
        coeffs[1][6] as i64,
        coeffs[2][6] as i64,
    );
    let rock_vel = Point::new(
        coeffs[3][6] as i64,
        coeffs[4][6] as i64,
        coeffs[5][6] as i64,
    );

    let ans = (-1..=1)
        .combinations_with_replacement(6)
        .map(|indices| {
            let (dx, dy, dz, dvx, dvy, dvz) = (
                indices[0], indices[1], indices[2], indices[3], indices[4], indices[5],
            );
            let hpos = stones[0].position;
            let hvel = stones[0].velocity;

            let pos_diff = Point::new(
                hpos.x - rock_pos.x - dx,
                hpos.y - rock_pos.y - dy,
                hpos.z - rock_pos.z - dz,
            );
            let vel_diff = Point::new(
                hvel.x - rock_vel.x - dvx,
                hvel.y - rock_vel.y - dvy,
                hvel.z - rock_vel.z - dvz,
            );

            let pos_diffx = cross(&pos_diff);
            (
                matvecmul(pos_diffx, [vel_diff.x, vel_diff.y, vel_diff.z]),
                (dx, dy, dz),
            )
        })
        .filter(|(prod, _)| prod[0] == 0 && prod[1] == 0 && prod[2] == 0)
        .map(|(_, (dx, dy, dz))| rock_pos.x + rock_pos.y + rock_pos.z + dx + dy + dz)
        .next()
        .unwrap();

    ans
}

pub fn never_fail_to_tell_me_the_odds_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let lines = reader.lines().filter_map(std::result::Result::ok);
    let ans = calc_intersection_pt(lines);

    println!("Answer: {ans}");
}
