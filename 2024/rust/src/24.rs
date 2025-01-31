use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/24.txt").expect("Failed to read file");

    let mut wires: HashMap<&str, u8> = HashMap::new();
    let mut gates: Vec<(&str, &str, &str, &str)> = Vec::new();

    for line in input.trim().lines() {
        let line = line.trim();
        if line.contains(":") {
            let parts: Vec<&str> = line.split(": ").collect();
            wires.insert(parts[0], parts[1].parse().unwrap());
        } else if line.contains("->") {
            let parts: Vec<&str> = line.split(" ").collect();
            gates.push((parts[0], parts[1], parts[2], parts[4]));
        }
    }

    let mut processed = true;
    while processed {
        processed = false;
        let mut remaining_gates = Vec::new();

        for (input1, gate, input2, output) in gates {
            if let (Some(&val1), Some(&val2)) = (wires.get(input1), wires.get(input2)) {
                let result = match gate {
                    "AND" => val1 & val2,
                    "OR" => val1 | val2,
                    "XOR" => val1 ^ val2,
                    _ => panic!("Unknown gate"),
                };
                if !wires.contains_key(output) {
                    wires.insert(output, result);
                }
                processed = true;
            } else {
                remaining_gates.push((input1, gate, input2, output));
            }
        }

        gates = remaining_gates;
    }

    let mut result = String::new();
    for i in 0.. {
        let wire = format!("z{:02}", i);
        if let Some(&value) = wires.get(&wire[..]) {
            result.push_str(&value.to_string());
        } else {
            break;
        }
    }

    let decimal_result =
        usize::from_str_radix(&result.chars().rev().collect::<String>(), 2).unwrap();
    println!("Resulting binary number: {}", result);
    println!("Decimal value: {}", decimal_result);
}
