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

const CHEAT_DURATION: i32 = 20;
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const SAVE_LIMIT: u32 = 100;

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
    let boundary_x = map[0].len() as i32;
    let boundary_y = map.len() as i32;
    let mut visited = HashMap::new();

    let mut current_cost = 0;
    for &(x, y) in path {
        println!("{}", current_cost);

        // Iterate through all possible positions within a diamond-shaped boundary
        for dx in 0..=CHEAT_DURATION + 1 {
            for dy in 0..=CHEAT_DURATION + 1 {
                let positions_to_check: Vec<(i32, i32)> = vec![
                    (x as i32 - dx, y as i32 - dy),
                    (x as i32 - dx, y as i32 + dy),
                    (x as i32 + dx, y as i32 - dy),
                    (x as i32 + dx, y as i32 + dy),
                ];

                for pos in positions_to_check {
                    let nx = pos.0 as i32;
                    let ny = pos.1 as i32;
                    if visited.contains_key(&(x, y, nx, ny)) {
                        continue;
                    }
                    visited.insert((x, y, nx, ny), true);
                    if nx < boundary_x && ny < boundary_y && nx >= 0 && ny >= 0 {
                        let distance = (dx + dy) as u32;
                        if distance < 2 || distance > CHEAT_DURATION as u32 {
                            continue;
                        }
                        if let Some(&cost) = path_cost.get(&(nx as usize, ny as usize)) {
                            let new_cost = current_cost + distance + cost;
                            if new_cost <= max_cost - SAVE_LIMIT {
                                shortcuts.push(max_cost - new_cost);
                            }
                        }
                    }
                }
            }
        }

        current_cost += 1;
    }

    // println!("Shortcuts: {:?}", shortcuts);

    // group shortcuts by cost, sort them by cost and println
    // let mut grouped_shortcuts: HashMap<u32, Vec<u32>> = HashMap::new();
    // for &shortcut in &shortcuts {
    //     grouped_shortcuts
    //         .entry(shortcut)
    //         .or_insert_with(Vec::new)
    //         .push(shortcut);
    // }

    // for (_, shortcuts) in grouped_shortcuts.iter_mut() {
    //     shortcuts.sort_unstable();
    // }

    // for (cost, shortcuts) in grouped_shortcuts.iter() {
    //     println!("{}, {}", shortcuts.len() as u32, cost);
    // }

    // return the number of shortcuts

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
