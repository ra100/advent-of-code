use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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

    if let Some((path, moves, total_cost)) = find_path_through_maze(&input) {
        println!("Path found: {:?}", path);
        println!("Moves: {:?}", moves);
        println!("Total cost: {}", total_cost);
    } else {
        println!("No path found from S to E.");
    }
}

fn render_map(map: &Vec<Vec<char>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn find_path_through_maze(map: &Vec<Vec<char>>) -> Option<(Vec<(usize, usize)>, Vec<&str>, i32)> {
    let start = find_position(map, 'S')?;
    let end = find_position(map, 'E')?;
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
            let total_cost = cost;
            let mut current = position;
            while let Some(&(prev, dir)) = came_from.get(&current) {
                path.push(current);
                moves.push(dir.to_string());
                current = prev;
            }
            path.push(start);
            path.reverse();
            moves.reverse();
            return Some((path, moves, total_cost));
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

    None
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
