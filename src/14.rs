use std::fs;

use regex::Regex;

const MAP_X: usize = 101;
const MAP_Y: usize = 103;
const ITERATIONS: u32 = 100;

fn main() {
    let content = fs::read_to_string("src/14.txt").expect("Failed to read file");
    let robots = parse_content(&content);

    let mut map = vec![vec![0; MAP_X]; MAP_Y];

    for robot in robots {
        let x = robot.0;
        let y = robot.1;
        let vx = robot.2;
        let vy = robot.3;

        let new_pos_x = x + vx * ITERATIONS as i64;
        let new_pos_y = y + vy * ITERATIONS as i64;

        let mut pos_x = new_pos_x % MAP_X as i64;
        let mut pos_y = new_pos_y % MAP_Y as i64;

        if pos_x < 0 {
            pos_x = MAP_X as i64 + pos_x;
        }

        if pos_y < 0 {
            pos_y = MAP_Y as i64 + pos_y;
        }

        // println!("{} {} {} {}: {} {}", x, y, vx, vy, pos_x, pos_y);

        map[pos_y as usize][pos_x as usize] += 1;
    }

    let mut quadrants = vec![0; 4];

    for i in 0..MAP_Y {
        for j in 0..MAP_X {
            if i < MAP_Y / 2 && j < MAP_X / 2 {
                quadrants[0] += map[i][j];
                continue;
            }
            if i < MAP_Y / 2 && j > MAP_X / 2 {
                quadrants[1] += map[i][j];
                continue;
            }
            if i > MAP_Y / 2 && j < MAP_X / 2 {
                quadrants[2] += map[i][j];
                continue;
            }
            if i > MAP_Y / 2 && j > MAP_X / 2 {
                quadrants[3] += map[i][j];
                continue;
            }
        }
    }

    println!("{:?}", quadrants);

    let result = quadrants.iter().copied().reduce(|a, b| a * b);

    println!("{}", result.unwrap());

    // for i in 0..MAP_Y {
    //     for j in 0..MAP_X {
    //         print!("{} ", map[i][j]);
    //     }
    //     println!();
    // }
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
