use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

fn main() {
    let input = fs::read_to_string("src/24.txt").expect("Failed to read file");

    let mut initial_wires: HashMap<&str, u8> = HashMap::new();
    let mut initial_gates: Vec<(&str, &str, &str, &str)> = Vec::new();
    let mut max_bit_index: usize = 0;

    for line in input.trim().lines() {
        let line = line.trim();
        if line.contains(":") {
            let parts: Vec<&str> = line.split(": ").collect();
            initial_wires.insert(parts[0], parts[1].parse().unwrap());
        } else if line.contains("->") {
            let parts: Vec<&str> = line.split(" ").collect();
            initial_gates.push((parts[0], parts[1], parts[2], parts[4]));
            if parts[4].starts_with("z") {
                let number: Vec<&str> = parts[4].split("z").collect();
                let number: usize = number[1].parse().unwrap();
                max_bit_index = max_bit_index.max(number);
            }
        }
    }

    max_bit_index -= 2;
    println!("Bit count: {}", max_bit_index);

    // Expected output for addition
    let x_bits: Vec<u8> = (0..max_bit_index)
        .map(|i| *initial_wires.get(&format!("x{:02}", i)[..]).unwrap())
        .collect();
    let y_bits: Vec<u8> = (0..max_bit_index)
        .map(|i| *initial_wires.get(&format!("y{:02}", i)[..]).unwrap())
        .collect();
    println!("x: {:?}", x_bits);
    println!("y: {:?}", y_bits);
    let x_value = bits_to_number(&x_bits);
    let y_value = bits_to_number(&y_bits);
    let expected_sum = x_value + y_value;
    let expected_bits = number_to_bits(expected_sum, max_bit_index);

    println!("Expected sum: {}", expected_sum);

    let start = Instant::now();
    let discrepancies = find_discrepancies(
        &initial_wires,
        &initial_gates,
        max_bit_index,
        &expected_bits,
    );
    let x_discrepancies = discrepancies
        .iter()
        .map(|d| format!("x{}", d))
        .collect::<Vec<_>>();
    let y_discrepancies = discrepancies
        .iter()
        .map(|d| format!("y{}", d))
        .collect::<Vec<_>>();
    let z_discrepancies = discrepancies
        .iter()
        .map(|d| format!("z{}", d))
        .collect::<Vec<_>>();
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    // find invalid gates
    let mut invalid_gates_forward = Vec::new();
    let mut invalid_gates = Vec::new();
    let mut wires_to_follow = VecDeque::new();
    let mut z_wires_to_follow = VecDeque::new();

    for &(input1, gate, input2, output) in &initial_gates {
        if y_discrepancies.contains(&input1.to_string())
            || y_discrepancies.contains(&input2.to_string())
            || x_discrepancies.contains(&input1.to_string())
            || x_discrepancies.contains(&input2.to_string())
        {
            invalid_gates_forward.push((input1, gate, input2, output));
            wires_to_follow.push_back(output);
        }

        if z_discrepancies.contains(&output.to_string()) {
            invalid_gates.push((input1, gate, input2, output));
            z_wires_to_follow.push_back(input1);
            z_wires_to_follow.push_back(input2);
        }
    }

    while let Some(wire) = wires_to_follow.pop_front() {
        for &(input1, gate, input2, output) in &initial_gates {
            if (input1 == wire || input2 == wire)
                && !invalid_gates_forward.contains(&(input1, gate, input2, output))
            {
                invalid_gates_forward.push((input1, gate, input2, output));
                wires_to_follow.push_back(output);
            }
        }
    }

    // println!(
    //     "Invalid gates: {} {:?}",
    //     invalid_gates_forward.len(),
    //     invalid_gates_forward
    // );

    while let Some(wire) = z_wires_to_follow.pop_front() {
        for &(input1, gate, input2, output) in &initial_gates {
            if output == wire
                && !invalid_gates.contains(&(input1, gate, input2, output))
                && invalid_gates_forward.contains(&(input1, gate, input2, output))
            {
                invalid_gates.push((input1, gate, input2, output));
                z_wires_to_follow.push_back(input1);
                z_wires_to_follow.push_back(input2);
            }
        }
    }

    // println!("Invalid gates: {} {:?}", invalid_gates.len(), invalid_gates);

    let mut swapped_wires = HashSet::new();
    let mut useless_wires = HashSet::new();
    let mut modified_gates = initial_gates.clone();
    let possible_gates = invalid_gates.clone();

    for &invalid_gate in &possible_gates {
        let mut found = false;
        if useless_wires.contains(invalid_gate.3) {
            continue;
        }
        for &gate_to_swap in &possible_gates {
            if gate_to_swap == invalid_gate {
                continue;
            }
            if useless_wires.contains(gate_to_swap.3) {
                continue;
            }
            let mut new_gates: Vec<(&str, &str, &str, &str)> = modified_gates
                .iter()
                .filter(|&item| item.3 == invalid_gate.3 || item.3 == gate_to_swap.3)
                .cloned()
                .collect();
            new_gates.push((
                gate_to_swap.0,
                gate_to_swap.1,
                gate_to_swap.2,
                invalid_gate.3,
            ));
            new_gates.push((
                invalid_gate.0,
                invalid_gate.1,
                invalid_gate.2,
                gate_to_swap.3,
            ));

            let new_discrepancies =
                find_discrepancies(&initial_wires, &new_gates, max_bit_index, &expected_bits);

            if new_discrepancies.len() <= discrepancies.len() {
                println!("Swapping {} and {}", invalid_gate.3, gate_to_swap.3);
                println!(
                    "New discrepancies: {} {}",
                    new_discrepancies.len(),
                    discrepancies.len()
                );
                swapped_wires.insert(invalid_gate.3);
                swapped_wires.insert(gate_to_swap.3);
                modified_gates = new_gates;
                found = true;
            }
        }
        if !found {
            useless_wires.insert(invalid_gate.3);
        }
    }

    println!("Swapped wires: {:?}", swapped_wires);
}

fn find_discrepancies(
    initial_wires: &HashMap<&str, u8>,
    initial_gates: &Vec<(&str, &str, &str, &str)>,
    max_bit_index: usize,
    expected_bits: &Vec<u8>,
) -> Vec<String> {
    let mut wires = initial_wires.clone();
    let mut gates = initial_gates.clone();
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
                wires.insert(output, result);
                processed = true;
            } else {
                remaining_gates.push((input1, gate, input2, output));
            }
        }

        gates = remaining_gates;
    }

    // Compare actual output with expected output
    let mut discrepancies = Vec::new();
    for i in 0..max_bit_index {
        let wire = format!("z{:02}", i);
        if let Some(&actual_value) = wires.get(&wire[..]) {
            if actual_value != expected_bits[i] {
                discrepancies.push(wire.split_at(1).1.to_string());
            }
        }
    }

    println!("Discrepancies: {:?}", discrepancies);
    discrepancies
}

fn bits_to_number(bits: &[u8]) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, &bit)| (bit as u64) << i)
        .sum()
}

fn number_to_bits(mut number: u64, bit_count: usize) -> Vec<u8> {
    let mut bits = vec![0; bit_count];
    for i in 0..bit_count {
        bits[i] = (number & 1) as u8;
        number >>= 1;
    }
    bits
}
