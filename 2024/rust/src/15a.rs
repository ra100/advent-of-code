use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Wall,
    CrateLeft,
    CrateRight,
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
            '[' => Cell::CrateLeft,
            ']' => Cell::CrateRight,
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
    let path = Path::new("src/15a.txt");
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
            let new_line = line
                .replace("#", "##")
                .replace(".", "..")
                .replace("O", "[]")
                .replace("@", "@.");
            map.push(parse_map_line(&new_line));
        } else {
            directions.extend(parse_directions_line(&line));
        }
    }

    render_map(&map, (0, 0));

    process_map(&mut map, &directions);

    let result = render_map(&map, (0, 0));

    println!("result: {:?}", result);
    // println!("Directions: {:?}", directions);
}

fn render_map(map: &Vec<Vec<Cell>>, pos: (usize, usize)) -> usize {
    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if i == pos.1 && j == pos.0 {
                print!("@");
                continue;
            }
            match map[i][j] {
                Cell::Wall => print!("#"),
                Cell::CrateLeft => {
                    result += 100 * i + j;
                    print!("[");
                }
                Cell::CrateRight => {
                    print!("]");
                }
                Cell::Empty => print!("."),
                Cell::Initial => print!("@"),
            }
        }
        println!();
    }

    return result;
}

fn process_map(map: &mut Vec<Vec<Cell>>, directions: &Vec<Direction>) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Cell::Initial = cell {
                x = j as usize;
                y = i as usize;
            }
        }
    }

    map[y][x] = Cell::Empty;

    for direction in directions {
        // println!("Direction: {:?}", direction);
        let (dx, dy) = get_direction(direction);
        let horizontal = dy == 0;
        let mut can_move = false;

        if horizontal {
            let mut nx = x as i32;

            loop {
                nx = nx + dx;
                let cell = &map[y as usize][nx as usize];
                match cell {
                    Cell::Empty => {
                        can_move = true;
                        break;
                    }
                    Cell::Wall => break,
                    _ => continue,
                }
            }

            if !can_move {
                continue;
            }

            while nx != x as i32 - dx {
                let nnx = (nx - dx) as usize;
                map[y as usize][nx as usize] = map[y as usize][nnx].clone();
                nx -= dx;
            }

            map[y as usize][x as usize] = Cell::Empty;
            x = (x as i32 + dx) as usize;
            map[y as usize][x as usize] = Cell::Empty;

            // println!("x: {}, y: {}", x, y);
            // render_map(map, (x, y));
            continue;
        }

        let mut to_push: Vec<(usize, usize)> = vec![(x as usize, (y as i32 + dy) as usize)];
        let mut crates: Vec<(Cell, (usize, usize))> = vec![];

        can_move = true;

        while to_push.len() > 0 {
            let (px, py) = to_push.pop().unwrap();
            let cell = &map[py][px];
            match cell {
                Cell::Wall => {
                    can_move = false;
                    break;
                }
                Cell::Empty => continue,
                Cell::CrateLeft => {
                    crates.push((Cell::CrateLeft, (px, py)));
                    crates.push((Cell::CrateRight, (px + 1, py)));
                    to_push.push((px, (py as i32 + dy) as usize));
                    to_push.push((px + 1, (py as i32 + dy) as usize));
                }
                Cell::CrateRight => {
                    crates.push((Cell::CrateRight, (px, py)));
                    crates.push((Cell::CrateLeft, (px - 1, py)));
                    to_push.push((px, (py as i32 + dy) as usize));
                    to_push.push((px - 1, (py as i32 + dy) as usize));
                }
                _ => continue,
            }
        }

        if !can_move {
            // println!("Can't move");
            // render_map(map, (x, y));
            continue;
        }

        println!("Crates: {:?}", crates);

        for (_, (cx, cy)) in crates.clone().into_iter() {
            map[cy][cx] = Cell::Empty;
        }

        for (cell, (cx, cy)) in crates {
            map[(cy as i32 + dy) as usize][cx] = cell;
        }

        map[y as usize][x as usize] = Cell::Empty;
        y = (y as i32 + dy) as usize;
        map[y as usize][x as usize] = Cell::Empty;

        // println!("x: {}, y: {}", x, y);
        // render_map(map, (x, y));
    }

    return (x, y);
}

fn get_direction(d: &Direction) -> (i32, i32) {
    match d {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Up => (0, -1),
    }
}
