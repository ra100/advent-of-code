use std::fs;

use regex::Regex;

const MAP_X: usize = 101;
const MAP_Y: usize = 103;
const ITERATIONS: u32 = 1000000;

fn main() {
    let content = fs::read_to_string("src/14.txt").expect("Failed to read file");
    let robots = parse_content(&content);

    for iter in 0..ITERATIONS {
        let mut map = vec![vec![0; MAP_X]; MAP_Y];
        let mut in_tree = 0;
        for robot in &robots {
            let x = robot.0;
            let y = robot.1;
            let vx = robot.2;
            let vy = robot.3;

            let new_pos_x = x + vx * iter as i64;
            let new_pos_y = y + vy * iter as i64;

            let mut pos_x = new_pos_x % MAP_X as i64;
            let mut pos_y = new_pos_y % MAP_Y as i64;

            if pos_x < 0 {
                pos_x = MAP_X as i64 + pos_x;
            }

            if pos_y < 0 {
                pos_y = MAP_Y as i64 + pos_y;
            }

            if is_tree(pos_x as usize, pos_y as usize) {
                in_tree += 1;
                map[pos_y as usize][pos_x as usize] += 1;
            }
        }

        if in_tree > (robots.len() as f32 / 1.2) as u32 {
            println!("Iteration: {}", iter);
            draw_map(&map);
            break;
        }
    }
}

fn is_tree(x: usize, y: usize) -> bool {
    let center_x = MAP_X / 2;

    // Check if the position is within the bounds of the tree
    let width_at_y = y / 2 + 1;
    let left_bound = center_x.saturating_sub(width_at_y);
    let right_bound = center_x + width_at_y;
    return x >= left_bound && x <= right_bound;
}

fn draw_map(map: &Vec<Vec<i64>>) {
    for i in 0..MAP_Y {
        for j in 0..MAP_X {
            match map[i][j] {
                0 => print!("."),
                _ => {
                    print!("{}", map[i][j])
                }
            };
        }
        println!();
    }
}

fn parse_content(content: &str) -> Vec<(i64, i64, i64, i64)> {
    let lines = content.lines();
    let mut result = Vec::new();

    for line in lines {
        if line.len() < 1 {
            continue;
        }
        let coordinates = parse_coordinates(line, r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)");
        result.push(coordinates);
    }

    return result;
}

fn parse_coordinates(line: &str, pattern: &str) -> (i64, i64, i64, i64) {
    let re = Regex::new(pattern).unwrap();
    let caps = re.captures(line).unwrap();

    let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
    let v = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
    let w = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

    (x, y, v, w)
}
