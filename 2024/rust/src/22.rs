use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
    fs,
};

const MODULO: i64 = 16777216;

fn main() {
    let content = fs::read_to_string("src/22.txt").expect("Failed to read file");

    let input: Vec<i64> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut sum = 0;
    let mut prices = vec![];
    let mut sequences: Vec<HashMap<(i16, i16, i16, i16), i16>> = Vec::new();

    for row in &input {
        let mut sequences_map: HashMap<(i16, i16, i16, i16), i16> = HashMap::new();
        let mut differences: VecDeque<i16> = VecDeque::with_capacity(4);
        let mut row_prices: Vec<i16> = vec![];
        let mut a = *row;
        let mut previous_price = (a % 10) as i16;
        let mut diff;
        let mut price;
        row_prices.push(previous_price);
        for _ in 0..2000 {
            a = ((a * 64) ^ a) % MODULO;
            a = ((a / 32) ^ a) % MODULO;
            a = ((a * 2048) ^ a) % MODULO;

            price = (a % 10) as i16;
            row_prices.push(price);
            diff = (price - previous_price) as i16;
            if differences.len() == 4 {
                differences.pop_front();
                differences.push_back(diff);
                let key = (
                    differences[0],
                    differences[1],
                    differences[2],
                    differences[3],
                );
                if !sequences_map.contains_key(&key) {
                    sequences_map.insert(key, price as i16);
                }
            } else {
                differences.push_back(diff);
            }

            previous_price = price;
        }
        sum += a;
        prices.push(row_prices);
        sequences.push(sequences_map);
    }

    println!("Sum: {}", sum);
    println!("Prices: {:?}", sequences[0].len());

    // Dynamic programming table to store the maximum sum for each key
    let mut dp: HashMap<(i16, i16, i16, i16), i16> = HashMap::new();

    let mut max_sum = 0;

    // Iterate through each sequence and update the dp table
    for sequence in &sequences {
        println!("Sequence: {:?}", sequence.len());
        for (key, _) in sequence {
            if dp.contains_key(&key) {
                continue;
            }
            let mut sum = 0;
            for other_sequence in &sequences {
                let val = other_sequence.get(&key).cloned().unwrap_or(0);
                sum += val;
            }
            dp.insert(*key, sum);
            max_sum = max(max_sum, sum);
        }
    }

    println!("Largest sum: {}", max_sum);
}
