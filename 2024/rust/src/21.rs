use std::{collections::HashMap, fs};

// 163872
// 163872
// 162740
// 171156

fn main() {
    let content = fs::read_to_string("src/21.txt").expect("Failed to read file");

    let input: Vec<(i32, Vec<char>)> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let num = line.split("A").collect::<Vec<&str>>()[0];
            let num = num.parse::<i32>().unwrap();
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

    let mut key_map_num: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    let mut key_map_arrow: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();

    find_key_combination(&num_pad, &mut key_map_num, dead_number);
    find_key_combination(&arrow_pad, &mut key_map_arrow, dead_arrow);

    let mut sequences = Vec::new();

    for row in &input {
        let number_sequences = get_button_sequence(row.1.clone(), &key_map_num, 'A');
        let mut arrow_sequences_1 = Vec::new();
        for seq in &number_sequences {
            print_array(seq);
            arrow_sequences_1.extend(get_button_sequence(seq.clone(), &key_map_arrow, 'A'));
        }

        let mut arrow_sequences_2 = Vec::new();
        for seq in &arrow_sequences_1 {
            arrow_sequences_2.extend(get_button_sequence(seq.clone(), &key_map_arrow, 'A'));
            print_array(seq);
        }

        let min_length = arrow_sequences_2
            .iter()
            .map(|seq| seq.len())
            .min()
            .unwrap_or(0);

        sequences.push((row.0, min_length));
    }

    let mut complexity = 0;
    for (i, sequence) in sequences.iter().enumerate() {
        complexity += sequence.1 as i32 * sequence.0;
        println!("Sequence {}: {}", i + 1, sequence.1);
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

    // let min_length = button_sequences
    //     .iter()
    //     .map(|seq| seq.len())
    //     .min()
    //     .unwrap_or(0);

    // // Filter sequences to include only those with the shortest length
    // button_sequences
    //     .into_iter()
    //     .filter(|seq| seq.len() == min_length)
    //     .collect()
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

            key_map.insert((*first_key, *second_key), combinations);
        }
    }
}
