use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("src/12.txt")
        .expect("Failed to read file")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    println!("{:?}", input);

    let mut results: HashMap<String, (u32, u32, u32)> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut region_count: HashMap<char, u32> = HashMap::new();

    let rows = input.len();
    let cols = input[0].len();
    let mut string_matrix = vec![vec![String::new(); cols]; rows];
    let mut perimeter_matrix: Vec<Vec<(bool, bool, bool, bool)>> =
        vec![vec![(false, false, false, false); cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                let letter = input[i][j];
                let region_id = region_count.entry(letter).or_insert(0);
                *region_id += 1;
                let new_letter = format!("{}{}", letter, region_id);
                flood_fill(
                    &input,
                    &mut string_matrix,
                    &mut visited,
                    i,
                    j,
                    letter,
                    &new_letter,
                );
            }
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            let letter = &string_matrix[i][j];
            let entry = results.entry(letter.to_string()).or_insert((0, 0, 0));
            entry.1 += 1; // Increment area

            // Check perimeter
            // up
            if i == 0 || string_matrix[i - 1][j] != *letter {
                entry.0 += 1;
                perimeter_matrix[i][j].0 = true;
            }
            // down
            if i == rows - 1 || string_matrix[i + 1][j] != *letter {
                entry.0 += 1;
                perimeter_matrix[i][j].2 = true;
            }
            // left
            if j == 0 || string_matrix[i][j - 1] != *letter {
                entry.0 += 1;
                perimeter_matrix[i][j].3 = true;
            }
            // right
            if j == cols - 1 || string_matrix[i][j + 1] != *letter {
                entry.0 += 1;
                perimeter_matrix[i][j].1 = true;
            }
        }
    }

    let mut visited_matrix: Vec<Vec<(bool, bool, bool, bool)>> =
        vec![vec![(false, false, false, false); cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            let letter = &string_matrix[i][j];
            for direction in 0..4 {
                match direction {
                    0 => {
                        if !perimeter_matrix[i][j].0 || visited_matrix[i][j].0 {
                            continue;
                        }
                    }
                    1 => {
                        if !perimeter_matrix[i][j].1 || visited_matrix[i][j].1 {
                            continue;
                        }
                    }
                    2 => {
                        if !perimeter_matrix[i][j].2 || visited_matrix[i][j].2 {
                            continue;
                        }
                    }
                    3 => {
                        if !perimeter_matrix[i][j].3 || visited_matrix[i][j].3 {
                            continue;
                        }
                    }
                    _ => break,
                }

                // println!(
                //     "Checking for letter: {}, i: {}, j: {}, direction: {}",
                //     letter, i, j, direction
                // );

                let new_fence = count_fences(
                    &string_matrix,
                    &perimeter_matrix,
                    &mut visited_matrix,
                    i,
                    j,
                    direction,
                    letter,
                );
                let entry = results.entry(letter.to_string()).or_insert((0, 0, 0));
                entry.2 += new_fence;
            }
        }
    }

    let mut result: u32 = 0;
    for (_, (_, area, sides)) in &results {
        result += area * sides;
    }

    for (letter, (perimeter, area, sides)) in &results {
        println!(
            "Letter: {}, Perimeter: {}, Area: {}, Sides: {}",
            letter, perimeter, area, sides
        );
    }

    println!("Result: {}", result);
}

fn flood_fill(
    input: &Vec<Vec<char>>,
    string_matrix: &mut Vec<Vec<String>>,
    visited: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    target: char,
    replacement: &str,
) {
    let rows = input.len();
    let cols = input[0].len();
    let mut stack = vec![(x, y)];

    while let Some((i, j)) = stack.pop() {
        if i >= rows || j >= cols || visited.contains(&(i, j)) || input[i][j] != target {
            continue;
        }

        visited.insert((i, j));
        string_matrix[i][j] = replacement.to_string();

        if i > 0 {
            stack.push((i - 1, j));
        }
        if i < rows - 1 {
            stack.push((i + 1, j));
        }
        if j > 0 {
            stack.push((i, j - 1));
        }
        if j < cols - 1 {
            stack.push((i, j + 1));
        }
    }
}

fn count_fences(
    string_matrix: &Vec<Vec<String>>,
    perimeter_matrix: &Vec<Vec<(bool, bool, bool, bool)>>,
    visited_matrix: &mut Vec<Vec<(bool, bool, bool, bool)>>,
    x: usize,
    y: usize,
    direction: usize,
    letter: &String,
) -> u32 {
    let rows = string_matrix.len();
    let cols = string_matrix[0].len();
    let mut stack = vec![(x, y)];

    while let Some((i, j)) = stack.pop() {
        if i >= rows || j >= cols || &string_matrix[i][j] != letter {
            continue;
        }

        match direction {
            0 => {
                if perimeter_matrix[i][j].0 {
                    if visited_matrix[i][j].0 {
                        0;
                    }
                    visited_matrix[i][j].0 = true;
                    stack.push((i, j + 1));
                }
            }
            1 => {
                if perimeter_matrix[i][j].1 {
                    if visited_matrix[i][j].1 {
                        0;
                    }
                    visited_matrix[i][j].1 = true;
                    stack.push((i + 1, j));
                }
            }
            2 => {
                if perimeter_matrix[i][j].2 {
                    if visited_matrix[i][j].2 {
                        0;
                    }
                    visited_matrix[i][j].2 = true;
                    stack.push((i, j + 1));
                }
            }
            3 => {
                if perimeter_matrix[i][j].3 {
                    if visited_matrix[i][j].3 {
                        0;
                    }
                    visited_matrix[i][j].3 = true;
                    stack.push((i + 1, j));
                }
            }
            _ => return 0,
        }
    }

    1
}
