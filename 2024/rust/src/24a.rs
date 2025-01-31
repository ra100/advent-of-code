use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    time::Instant,
};

fn main() {
    let input = fs::read_to_string("src/24.txt").expect("Failed to read file");

    let mut initial_wires: HashMap<&str, u8> = HashMap::new();
    let mut initial_gates: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut max_bit_length: usize = 0;

    for line in input.trim().lines() {
        let line = line.trim();
        if line.contains(":") {
            let parts: Vec<&str> = line.split(": ").collect();
            initial_wires.insert(parts[0], parts[1].parse().unwrap());
        } else if line.contains("->") {
            let parts: Vec<&str> = line.split(" ").collect();
            initial_gates.insert(parts[4], (parts[0], parts[1], parts[2]));
            if parts[4].starts_with("z") {
                let number: Vec<&str> = parts[4].split("z").collect();
                let number: usize = number[1].parse().unwrap();
                max_bit_length = max_bit_length.max(number);
            }
        }
    }
    let max_x_length: usize = initial_wires.len() / 2;
    max_bit_length += 1;
    println!("Bit count: {}", max_bit_length);

    let static_gates = initial_gates.clone();

    // Expected output for addition
    let x_bits: Vec<u8> = (0..max_x_length)
        .map(|i| *initial_wires.get(&format!("x{:02}", i)[..]).unwrap())
        .rev()
        .collect();
    let y_bits: Vec<u8> = (0..max_x_length)
        .map(|i| *initial_wires.get(&format!("y{:02}", i)[..]).unwrap())
        .rev()
        .collect();
    let x_value = bits_to_number(&x_bits);
    let y_value = bits_to_number(&y_bits);
    let expected_sum = x_value + y_value;
    let expected_bits = number_to_bits(expected_sum, max_bit_length);
    println!("x: {:?}", x_bits);
    println!("y: {:?}", y_bits);
    println!("z: {:?}", expected_bits);

    println!("Expected sum: {}", expected_sum);

    let start = Instant::now();
    let discrepancies = find_discrepancies(
        &initial_wires,
        &static_gates,
        max_bit_length,
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
    println!("Discrepancies: {:?}", discrepancies);

    // find invalid gates
    let mut invalid_gates_forward = Vec::new();
    let mut invalid_gates = Vec::new();
    let mut wires_to_follow = VecDeque::new();
    let mut z_wires_to_follow = VecDeque::new();

    for (&output, &(input1, _, input2)) in static_gates.iter() {
        if y_discrepancies.contains(&input1.to_string())
            || y_discrepancies.contains(&input2.to_string())
            || x_discrepancies.contains(&input1.to_string())
            || x_discrepancies.contains(&input2.to_string())
        {
            invalid_gates_forward.push(output);
            wires_to_follow.push_back(output);
        }

        if z_discrepancies.contains(&output.to_string()) {
            invalid_gates.push(output);
            z_wires_to_follow.push_back(input1);
            z_wires_to_follow.push_back(input2);
        }
    }

    while let Some(wire) = wires_to_follow.pop_front() {
        for (&output, &(input1, _, input2)) in static_gates.iter() {
            if (input1 == wire || input2 == wire) && !invalid_gates_forward.contains(&output) {
                invalid_gates_forward.push(&output);
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
        for (&output, &(input1, _, input2)) in static_gates.iter() {
            if output == wire
                && !invalid_gates.contains(&output)
                && invalid_gates_forward.contains(&output)
            {
                invalid_gates.push(output);
                z_wires_to_follow.push_back(input1);
                z_wires_to_follow.push_back(input2);
            }
        }
    }

    // println!("Invalid gates: {} {:?}", invalid_gates.len(), invalid_gates);

    let mut tried_wires: HashSet<&str> = HashSet::new();
    let mut possible_swaps: HashMap<(&str, &str), isize> = HashMap::new();
    let modified_gates = static_gates.clone();
    let mut possible_gates: Vec<&str> = invalid_gates.clone();
    // let mut possible_gates: Vec<&str> = static_gates.keys().map(|&k| k).collect();
    possible_gates.sort();
    let discrepancies_length = discrepancies.len() as isize;
    // let mut shortest = discrepancies.len();

    for &out_wire in &possible_gates {
        println!("Checking wire: {}", out_wire);
        if tried_wires.contains(out_wire) {
            continue;
        }
        for &wire_to_swap in &possible_gates {
            if wire_to_swap == out_wire || tried_wires.contains(wire_to_swap) {
                continue;
            }

            let mut temp_gates = modified_gates.clone();
            let gate1 = *temp_gates.get(out_wire).unwrap();
            let gate2 = *temp_gates.get(wire_to_swap).unwrap();
            temp_gates.insert(wire_to_swap, gate1);
            temp_gates.insert(out_wire, gate2);

            let mod_discrepancies =
                find_discrepancies(&initial_wires, &temp_gates, max_bit_length, &expected_bits);

            if (mod_discrepancies.len() as isize) < discrepancies_length {
                // println!("Swapped {} with {}", out_wire, wire_to_swap);
                // println!("Discrepancies: {:?}", mod_discrepancies);
                possible_swaps.insert(
                    (out_wire, wire_to_swap),
                    discrepancies_length - mod_discrepancies.len() as isize,
                );
            }
        }
        tried_wires.insert(out_wire);
    }

    println!("Possible swaps: {:?}", possible_swaps.len());

    // let swapped_wires: HashSet<&str> = HashSet::new();

    // let mut swapped_wires_vec: Vec<&str> = swapped_wires.into_iter().collect();
    // swapped_wires_vec.sort();

    // println!(
    //     "Swapped wires: {} {:?}",
    //     swapped_wires_vec.len(),
    //     swapped_wires_vec.join(",")
    // );
}

fn find_discrepancies(
    initial_wires: &HashMap<&str, u8>,
    initial_gates: &HashMap<&str, (&str, &str, &str)>,
    max_bit_index: usize,
    expected_bits: &Vec<u8>,
) -> HashSet<String> {
    let mut wires = initial_wires.clone();
    let mut gates = initial_gates.clone();
    let mut processed = true;
    while processed {
        processed = false;
        let mut used_gates = Vec::new();
        for (&output, &(input1, gate, input2)) in gates.iter() {
            if let (Some(&val1), Some(&val2)) = (wires.get(input1), wires.get(input2)) {
                let result = match gate {
                    "AND" => val1 & val2,
                    "OR" => val1 | val2,
                    "XOR" => val1 ^ val2,
                    _ => panic!("Unknown gate"),
                };
                wires.insert(output, result);
                used_gates.push(output);
                processed = true;
            }
        }
        for gate in used_gates {
            gates.remove(gate);
        }
    }

    // Compare actual output with expected output
    let mut discrepancies: HashSet<String> = HashSet::new();
    for i in 0..max_bit_index {
        let wire = format!("z{:02}", i);
        if let Some(&actual_value) = wires.get(&wire[..]) {
            if actual_value != expected_bits[i] {
                discrepancies.insert(wire.split_at(1).1.to_string());
            }
        }
    }

    // println!("Discrepancies: {:?}", discrepancies);
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
