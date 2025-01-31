use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("src/20.txt").expect("Failed to read file");

    let map: Vec<Vec<char>> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let (path_cost, path) = find_path(&map);

    // for p in &path {
    //     println!("p: {:?} c: {:?}", p, path_cost.get(p).unwrap());
    //     map[p.1][p.0] = ' ';
    // }

    // for row in &map {
    //     println!("{}", row.iter().collect::<String>());
    // }

    let savings = find_shortcuts(&path_cost, &path, &map);

    println!("Path length: {}", path.len());
    // println!("Path: {:?}", path);
    // println!("Path cost: {:?}", path_cost);
    println!("Savings: {}", savings);
}

//837184

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const CHEAT_DIRECTIONS: [(i32, i32); 4] = [(0, 2), (0, -2), (2, 0), (-2, 0)];

fn find_path(map: &Vec<Vec<char>>) -> (HashMap<(usize, usize), u32>, Vec<(usize, usize)>) {
    let start = find_position(map, 'S').unwrap();
    let end = find_position(map, 'E').unwrap();
    let mut path_cost = HashMap::new();
    let mut path: Vec<(usize, usize)> = vec![];
    let mut stack = vec![start];

    while let Some(position) = stack.pop() {
        path.push(position);
        path_cost.insert((position.0, position.1), path.len() as u32);

        for (dx, dy) in DIRECTIONS.iter() {
            let x = position.0 as i32 + dx;
            let y = position.1 as i32 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if y >= map.len() || x >= map[0].len() {
                continue;
            }

            if path.contains(&(x, y)) {
                continue;
            }

            let char = map[y][x];

            if char == '.' || char == 'E' {
                if char != 'E' {
                    stack.push((x, y));
                }
                break;
            }
        }
    }

    let length = path.len();

    for (&position, cost) in path_cost.clone().iter() {
        path_cost.insert(position, length as u32 - cost + 1);
    }

    path_cost.insert(end, 0);

    (path_cost, path)
}

/**
 * Find where to cheat, where you can go through the wall to get short path.
 * To move through wall, it's 2 steps in one direction that ends up on empty space "."
 * Count all shortcuts that save more than 100 steps.
 */
fn find_shortcuts(
    path_cost: &HashMap<(usize, usize), u32>,
    path: &Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
) -> u32 {
    let max_cost = path.len() as u32;
    let mut shortcuts: Vec<u32> = vec![];
    let boundary_x = map[0].len();
    let boundary_y = map.len();

    let mut current_cost = 0;
    for position in path {
        for (dx, dy) in CHEAT_DIRECTIONS.iter() {
            let x = position.0 as i32 + dx;
            let y = position.1 as i32 + dy;

            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= boundary_x || y >= boundary_y {
                continue;
            }

            if !path_cost.contains_key(&(x, y)) {
                continue;
            }

            let rest_cost = path_cost.get(&(x, y)).unwrap();

            let cheat_saved = max_cost as i32 - (current_cost + 2 + rest_cost) as i32;
            if cheat_saved >= 100 {
                shortcuts.push(cheat_saved as u32);
            }
        }
        current_cost += 1;
    }

    println!("Shortcuts: {:?}", shortcuts);

    return shortcuts.len() as u32;
}

fn find_position(map: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((j, i));
            }
        }
    }
    None
}
