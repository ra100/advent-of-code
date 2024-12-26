use std::fs;

#[derive(PartialEq)]
enum Part {
    Keys,
    Locks,
}

fn main() {
    let input = fs::read_to_string("src/25.txt").expect("Failed to read file");

    let mut keys: Vec<[u8; 5]> = Vec::new();
    let mut locks: Vec<[u8; 5]> = Vec::new();

    let mut parts: [u8; 5] = [0, 0, 0, 0, 0];
    let mut target: Part = Part::Keys;
    let mut new_part = true;
    let mut row_index = 0;

    for line in input.trim().lines() {
        if line.len() == 0 {
            new_part = true;
            continue;
        }

        if new_part && line == "#####" {
            new_part = false;
            target = Part::Locks;
        }

        if new_part && line == "....." {
            new_part = false;
            target = Part::Keys;
        }

        if row_index == 6 {
            row_index = 0;
            if target == Part::Keys {
                keys.push(parts);
            } else {
                locks.push(parts);
            }
            parts = [0, 0, 0, 0, 0];
            continue;
        }

        line.chars().enumerate().for_each(|(i, chr)| match chr {
            '#' => {
                if target == Part::Keys {
                    parts[i] += 1
                }
            }
            '.' => {
                if target == Part::Locks {
                    parts[i] += 1
                }
            }
            _ => (),
        });

        row_index += 1;
    }

    println!("Keys: {:?}", keys);
    println!("Locks: {:?}", locks);

    let mut valid_keys = 0;

    for &lock in &locks {
        for &key in &keys {
            let mut found = true;
            for i in 0..5 {
                if key[i] <= lock[i] {
                    continue;
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                valid_keys += 1;
            }
        }
    }

    println!("Valid keys: {}", valid_keys);
}
