use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Cell {
    Wall,
    Crate,
    Empty,
    Initial,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

fn parse_map_line(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '#' => Cell::Wall,
            'O' => Cell::Crate,
            '.' => Cell::Empty,
            '@' => Cell::Initial,
            _ => panic!("Unknown character in map"),
        })
        .collect()
}

fn parse_directions_line(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => panic!("Unknown character in directions"),
        })
        .collect()
}

fn main() {
    let path = Path::new("src/15.txt");
    let file = File::open(&path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let mut map = Vec::new();
    let mut directions = Vec::new();
    let mut is_parsing_map = true;

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            is_parsing_map = false;
            continue;
        }

        if is_parsing_map {
            map.push(parse_map_line(&line));
        } else {
            directions.extend(parse_directions_line(&line));
        }
    }

    process_map(&mut map, &directions);

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                Cell::Wall => print!("#"),
                Cell::Crate => {
                    result += 100 * i + j;
                    print!("O");
                }
                Cell::Empty => print!("."),
                Cell::Initial => print!("@"),
            }
        }
        println!();
    }

    println!("result: {:?}", result);
    // println!("Directions: {:?}", directions);
}

fn process_map(map: &mut Vec<Vec<Cell>>, directions: &Vec<Direction>) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Initial = cell {
                x = j as i32;
                y = i as i32;
            }
        }
    }

    map[y as usize][x as usize] = Cell::Empty;

    for direction in directions {
        let (dx, dy) = get_direction(direction);
        let mut empties = 0;
        let mut crates = 0;
        let mut nx = x as i32;
        let mut ny = y as i32;

        // println!(" x: {},  y: {}", x, y);

        while empties == 0 {
            (nx, ny) = (nx + dx, ny + dy);
            // println!("nx: {}, ny: {}", nx, ny);
            let cell = &map[ny as usize][nx as usize];
            match cell {
                Cell::Empty => empties += 1,
                Cell::Crate => crates += 1,
                Cell::Wall => break,
                _ => break,
            }
        }

        // println!(
        //     "Empties: {}, Crates: {}, Direction: {:?}",
        //     empties, crates, direction
        // );

        let mut nx = x as i32;
        let mut ny = y as i32;
        let to_move = empties;

        while crates > 0 || empties > 0 {
            (nx, ny) = (nx + dx, ny + dy);
            if empties > 0 {
                map[ny as usize][nx as usize] = Cell::Empty;
                empties -= 1;
            } else if crates > 0 {
                map[ny as usize][nx as usize] = Cell::Crate;
                crates -= 1;
            }
        }

        x = x as i32 + (dx * to_move);
        y = y as i32 + (dy * to_move);
    }
}

fn get_direction(d: &Direction) -> (i32, i32) {
    match d {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Up => (0, -1),
    }
}
