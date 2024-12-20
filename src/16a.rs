use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

const COST_ROTATE: i32 = 1000;
const COST_MOVE: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_string(&self) -> &'static str {
        match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: (usize, usize),
    direction: Direction,
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

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("src/16.txt")
        .expect("Failed to read file")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    println!("INPUT MAP:");
    render_map(&input);
    println!("");

    let mut map = input.clone();

    if let Some((paths, _, total_cost)) = find_paths_through_maze(&input) {
        let unique_positions = get_unique_positions(&paths);
        for path in &paths {
            for &position in path {
                map[position.0][position.1] = 'O';
            }
        }
        println!("Paths found: {:?}", paths);
        // println!("Moves: {:?}", moves);
        println!("Total cost: {}", total_cost);
        println!("Unique positions: {}", unique_positions);
    } else {
        println!("No path found from S to E.");
    }

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

fn get_unique_positions(paths: &Vec<Vec<(usize, usize)>>) -> i32 {
    let mut unique_positions = HashSet::new();

    for path in paths {
        for &position in path {
            unique_positions.insert(position);
        }
    }

    unique_positions.len() as i32
}

fn find_paths_through_maze(
    map: &Vec<Vec<char>>,
) -> Option<(Vec<Vec<(usize, usize)>>, Vec<Vec<Direction>>, i32)> {
    let cost_cache = create_cost_cache(map);
    println!("Cost cache loaded");
    let start = find_position(map, 'S')?;
    let end = find_position(map, 'E')?;

    let mut paths = Vec::new();
    let mut moves = Vec::new();
    let mut min_cost = 90460;

    fn dfs(
        map: &Vec<Vec<char>>,
        position: (usize, usize),
        direction: Direction,
        end: (usize, usize),
        path: &mut Vec<(usize, usize)>,
        move_list: &mut Vec<Direction>,
        paths: &mut Vec<Vec<(usize, usize)>>,
        moves: &mut Vec<Vec<Direction>>,
        min_cost: &mut i32,
        cost: i32,
        cost_cache: &HashMap<(usize, usize), i32>,
    ) {
        if position == end {
            if cost < *min_cost {
                *min_cost = cost;
                paths.clear();
                moves.clear();
            }
            if cost == *min_cost {
                paths.push(path.clone());
                moves.push(move_list.clone());
            }
            println!("Path found, cost: {}", cost);
            return;
        }

        if cost > *min_cost {
            return;
        }

        let cc = *cost_cache.get(&position).unwrap_or(&i32::MAX);

        if cost + cc > *min_cost {
            return;
        }

        let x_dist = position.0 as i32 - end.0 as i32;
        let y_dist = position.1 as i32 - end.1 as i32;
        let turn = if x_dist > 0 && y_dist > 0 { 1000 } else { 0 };

        if x_dist.abs() + y_dist.abs() + cost + turn > *min_cost {
            return;
        }

        for &(new_direction, (dy, dx)) in &[
            (Direction::Right, (0, 1)),
            (Direction::Down, (1, 0)),
            (Direction::Left, (0, -1)),
            (Direction::Up, (-1, 0)),
        ] {
            let new_position = (
                (position.0 as isize + dy) as usize,
                (position.1 as isize + dx) as usize,
            );

            if new_position.0 < map.len()
                && new_position.1 < map[0].len()
                && map[new_position.0][new_position.1] != '#'
                && !path.contains(&new_position)
            {
                let rotation_cost = if new_direction == direction {
                    0
                } else {
                    COST_ROTATE
                };
                let new_cost = cost + rotation_cost + COST_MOVE;

                if new_cost > *min_cost {
                    return;
                }

                path.push(new_position);
                move_list.push(new_direction);

                dfs(
                    map,
                    new_position,
                    new_direction,
                    end,
                    path,
                    move_list,
                    paths,
                    moves,
                    min_cost,
                    new_cost,
                    cost_cache,
                );

                path.pop();
                move_list.pop();
            }
        }
    }

    let mut initial_path = vec![start];
    let mut initial_moves = vec![];

    dfs(
        map,
        start,
        Direction::Right,
        end,
        &mut initial_path,
        &mut initial_moves,
        &mut paths,
        &mut moves,
        &mut min_cost,
        0,
        &cost_cache,
    );

    if paths.is_empty() {
        None
    } else {
        Some((paths, moves, min_cost))
    }
}

fn find_position(map: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn create_cost_cache(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), i32> {
    let mut cache = HashMap::new();
    let end = find_position(map, 'E');
    if end == None {
        return cache;
    }

    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' || cell == 'S' {
                let cost = find_single_path_through_maze(map, (i, j), end.unwrap());
                cache.insert((i, j), cost);
            }
        }
    }

    cache
}

fn find_single_path_through_maze(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> i32 {
    let directions = [
        (Direction::Right, (0, 1)),
        (Direction::Down, (1, 0)),
        (Direction::Left, (0, -1)),
        (Direction::Up, (-1, 0)),
    ];

    let mut heap = BinaryHeap::new();
    let mut costs: HashMap<(usize, usize), i32> = HashMap::new();
    let mut came_from: HashMap<(usize, usize), ((usize, usize), Direction)> = HashMap::new();

    heap.push(State {
        position: start,
        direction: Direction::Right,
        cost: 0,
    });
    costs.insert(start, 0);

    while let Some(State {
        position,
        direction,
        cost,
    }) = heap.pop()
    {
        if position == end {
            let mut path = Vec::new();
            let mut moves = Vec::new();
            let total_cost = cost - COST_ROTATE;
            let mut current = position;
            while let Some(&(prev, dir)) = came_from.get(&current) {
                path.push(current);
                moves.push(dir.to_string());
                current = prev;
            }
            path.push(start);
            path.reverse();
            moves.reverse();
            return total_cost;
        }

        for &(new_direction, (dy, dx)) in &directions {
            let new_position = (
                (position.0 as isize + dy) as usize,
                (position.1 as isize + dx) as usize,
            );

            if new_position.0 < map.len()
                && new_position.1 < map[0].len()
                && map[new_position.0][new_position.1] != '#'
            {
                let rotation_cost = if new_direction == direction {
                    0
                } else {
                    COST_ROTATE
                };
                let new_cost = cost + rotation_cost + COST_MOVE;

                if new_cost < *costs.get(&new_position).unwrap_or(&i32::MAX) {
                    heap.push(State {
                        position: new_position,
                        direction: new_direction,
                        cost: new_cost,
                    });
                    costs.insert(new_position, new_cost);
                    came_from.insert(new_position, (position, new_direction));
                }
            }
        }
    }

    return i32::MAX;
}
