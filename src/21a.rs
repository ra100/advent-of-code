use std::{collections::HashMap, fs, time::Instant};

// 163872
// 163872
// 162740
// 171156

const DEFINED_LEVEL: i16 = 25; // 2 = 2 directional keypads

fn main() {
    let content = fs::read_to_string("src/21.txt").expect("Failed to read file");

    let input: Vec<(i64, Vec<char>)> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let num = line.split("A").collect::<Vec<&str>>()[0];
            let num = num.parse::<i64>().unwrap();
            return (num, line.chars().collect());
        })
        .collect();
    println!("Input: {:?}", input);

    let mut num_pad: HashMap<char, (i8, i8)> = HashMap::new();
    num_pad.insert('7', (0, 0));
    num_pad.insert('8', (1, 0));
    num_pad.insert('9', (2, 0));
    num_pad.insert('4', (0, 1));
    num_pad.insert('5', (1, 1));
    num_pad.insert('6', (2, 1));
    num_pad.insert('1', (0, 2));
    num_pad.insert('2', (1, 2));
    num_pad.insert('3', (2, 2));
    num_pad.insert('0', (1, 3));
    num_pad.insert('A', (2, 3));

    let mut arrow_pad: HashMap<char, (i8, i8)> = HashMap::new();
    arrow_pad.insert('^', (1, 0));
    arrow_pad.insert('A', (2, 0));
    arrow_pad.insert('<', (0, 1));
    arrow_pad.insert('v', (1, 1));
    arrow_pad.insert('>', (2, 1));

    let dead_number: (i8, i8) = (0, 3);
    let dead_arrow: (i8, i8) = (0, 0);

    let mut key_map: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut memo: HashMap<String, usize> = HashMap::new();

    find_key_combination(&num_pad, &mut key_map, dead_number);
    find_key_combination(&arrow_pad, &mut key_map, dead_arrow);

    // println!("Finding key combinations {:?}", key_map);

    let mut short_sequences: Vec<(i64, i64)> = Vec::new();

    let start = Instant::now();

    for row in &input {
        let levels = DEFINED_LEVEL; // Number of levels

        let shortest_sequences =
            find_shortest_sequence_length(row.1.clone(), levels, 'A', &mut memo, &key_map);

        short_sequences.push((row.0, shortest_sequences as i64));
    }

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    // println!("memo: {:?}", memo);

    let mut complexity = 0;
    for (i, sequence) in short_sequences.iter().enumerate() {
        complexity += sequence.1 as i64 * sequence.0;
        println!("Sequence {}: buttons {}", i + 1, sequence.1);
    }

    println!("Complexity: {}", complexity);
}

fn print_array(array: &Vec<char>) {
    let button_sequence_str: String = array
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", button_sequence_str);
}

fn get_memo_key(input_sequence: &Vec<char>, level: i16) -> String {
    return format!("{:?} {}", input_sequence, level);
}

fn find_shortest_sequence_length(
    input_sequence: Vec<char>,
    level: i16,
    start_key: char,
    memo: &mut HashMap<String, usize>,
    key_map: &HashMap<(char, char), Vec<Vec<char>>>,
) -> usize {
    let key = get_memo_key(&input_sequence, level);
    if let Some(&value) = memo.get(&key) {
        return value;
    }

    if level == 0 {
        let transformed_sequences = get_button_sequence(input_sequence.clone(), key_map, start_key);
        let min_length = transformed_sequences
            .iter()
            .map(|seq| seq.len())
            .min()
            .unwrap_or(0);
        memo.insert(key, min_length);
        return min_length;
    }

    let transformed_sequences = get_button_sequence(input_sequence.clone(), key_map, start_key);
    // println!("{:?}: {:?}", input_sequence, transformed_sequences);
    let mut min_length = usize::MAX;

    for transformed_sequence in &transformed_sequences {
        let mut sequences = vec![vec![]];

        for key in transformed_sequence {
            sequences.last_mut().unwrap().push(*key);
            if *key == 'A' {
                sequences.push(vec![]);
            }
        }

        let start_char_in_loop = 'A';

        let mut subset_sequence_length = 0;
        for subset in &sequences {
            let subset_length = find_shortest_sequence_length(
                subset.clone(),
                level - 1,
                start_char_in_loop,
                memo,
                &key_map,
            );

            subset_sequence_length += subset_length;
        }

        if min_length > subset_sequence_length {
            min_length = subset_sequence_length;
        }
    }

    memo.insert(key, min_length);
    min_length
}

fn get_button_sequence(
    input: Vec<char>,
    key_map: &HashMap<(char, char), Vec<Vec<char>>>,
    start_key: char,
) -> Vec<Vec<char>> {
    let mut button_sequences: Vec<Vec<char>> = vec![vec![]];
    let mut current_key: char = start_key;

    for key in input {
        if let Some(key_combinations) = key_map.get(&(current_key, key)) {
            let mut new_sequences: Vec<Vec<char>> = Vec::new();
            for sequence in button_sequences {
                for combination in key_combinations {
                    let mut new_sequence = sequence.clone();
                    new_sequence.extend(combination.iter().copied());
                    new_sequences.push(new_sequence);
                }
            }
            button_sequences = new_sequences;
        }
        current_key = key;
    }

    button_sequences
}

fn find_key_combination(
    num_pad: &HashMap<char, (i8, i8)>,
    key_map: &mut HashMap<(char, char), Vec<Vec<char>>>,
    dead_key: (i8, i8),
) {
    for first_key in num_pad.keys() {
        let (x1, y1) = num_pad.get(first_key).unwrap();
        for second_key in num_pad.keys() {
            if first_key == second_key {
                key_map.insert((*first_key, *second_key), vec![vec!['A']]);
                continue;
            }
            let (x2, y2) = num_pad.get(second_key).unwrap();
            let dx = x2 - x1;
            let dy = y2 - y1;

            let horizontal_corner = (*x2, *y1);
            let vertical_corner = (*x1, *y2);

            let mut combinations = Vec::new();

            if vertical_corner != dead_key && dy != 0 {
                let mut key_combination = Vec::new();

                if dy > 0 {
                    for _ in 0..dy {
                        key_combination.push('v');
                    }
                } else {
                    for _ in 0..dy.abs() {
                        key_combination.push('^');
                    }
                }

                if dx > 0 {
                    for _ in 0..dx {
                        key_combination.push('>');
                    }
                } else {
                    for _ in 0..dx.abs() {
                        key_combination.push('<');
                    }
                }
                key_combination.push('A');
                combinations.push(key_combination);
            }

            if horizontal_corner != dead_key && dx != 0 {
                let mut key_combination = Vec::new();
                if dx > 0 {
                    for _ in 0..dx {
                        key_combination.push('>');
                    }
                } else {
                    for _ in 0..dx.abs() {
                        key_combination.push('<');
                    }
                }

                if dy > 0 {
                    for _ in 0..dy {
                        key_combination.push('v');
                    }
                } else {
                    for _ in 0..dy.abs() {
                        key_combination.push('^');
                    }
                }
                key_combination.push('A');
                combinations.push(key_combination);
            }

            key_map.insert((*first_key, *second_key), combinations);
        }
    }
}
