use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

// const MAP_SIZE: usize = 7;
// const MAX_BYTES: u32 = 12;
const MAP_SIZE: usize = 71;
// const MAX_BYTES: u32 = 1024;

fn main() {
    let file = fs::read_to_string("src/18.txt").expect("Failed to read file");
    let mut map = vec![vec!['.'; MAP_SIZE]; MAP_SIZE];
    let mut blocks: Vec<(i32, i32)> = Vec::new();
    for line in file.lines() {
        if line.is_empty() {
            continue;
        }
        let coords: Vec<i32> = line
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let (x, y) = (coords[0], coords[1]);
        blocks.push((x, y));
    }

    // render_map(&map);

    let blocks_length = 0;

    let mut path = vec![];

    let mut left = blocks_length;
    let mut right = blocks.len() - 1;
    let mut mid = (left + right) / 2;

    while left <= right {
        mid = (left + right) / 2;

        path = find_single_path_through_maze(
            &map,
            &blocks,
            mid as u32,
            (0, 0),
            (MAP_SIZE - 1, MAP_SIZE - 1),
        );

        if path.len() == 0 {
            println!("NO  path, {}", mid);
            right = mid - 1;
        } else {
            println!("YES path, {}", mid);
            left = mid + 1;
        }
    }

    println!("No path found {:?}", blocks[mid as usize]);

    // for i in blocks_length..blocks.len() {
    //     path = find_single_path_through_maze(
    //         &map,
    //         &blocks,
    //         i as u32,
    //         (0, 0),
    //         (MAP_SIZE - 1, MAP_SIZE - 1),
    //     );

    //     println!("iteration: {}", i);

    //     if path.len() == 0 {
    //         println!("No path found {:?}", blocks[i as usize]);
    //         break;
    //     }
    // }

    for i in 0..blocks_length {
        map[blocks[i as usize].1 as usize][blocks[i as usize].0 as usize] = '#';
    }

    for step in &path {
        map[step.0][step.1] = 'O';
    }

    println!("Length: {}", path.len());
    println!("Path: {:?}", path);

    println!("result Map:");
    render_map(&map);
}

fn render_map(map: &Vec<Vec<char>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (usize, usize),
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn find_single_path_through_maze(
    map: &Vec<Vec<char>>,
    blocks: &Vec<(i32, i32)>,
    blocks_length: u32,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<(usize, usize), i32> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let used_blocks = blocks.split_at(blocks_length as usize).0;

    heap.push(State {
        position: start,
        cost: 0,
    });
    costs.insert(start, 0);

    while let Some(State { position, cost }) = heap.pop() {
        if position == end {
            let mut path = Vec::new();
            let mut current = position;
            while let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse();
            return path;
        }

        for &(dy, dx) in &DIRECTIONS {
            let ny = position.0 as i32 + dy;
            let nx = position.1 as i32 + dx;

            if nx < 0 || ny < 0 || nx >= map.len() as i32 || ny >= map[0].len() as i32 {
                continue;
            }

            let new_position = (ny as usize, nx as usize);

            if used_blocks.contains(&(new_position.1 as i32, new_position.0 as i32)) {
                continue;
            }

            let new_cost = cost + 1;

            if new_cost < *costs.get(&new_position).unwrap_or(&i32::MAX) {
                heap.push(State {
                    position: new_position,
                    cost: new_cost,
                });
                costs.insert(new_position, new_cost);
                came_from.insert(new_position, position);
            }
        }
    }

    return Vec::new();
}
