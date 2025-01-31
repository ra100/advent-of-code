use std::{fs, time::Instant};

fn main() {
    let content = fs::read_to_string("src/19.txt").expect("Failed to read file");

    let mut lines = content.lines();

    let towels: Vec<String> = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs: Vec<String> = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    let mut possible_designs = 0u64;

    let start = Instant::now();

    for design in &designs {
        possible_designs += count_combinations(&towels, &design);
        let duration = start.elapsed();
        println!("{:?}", duration);
    }

    println!("Possible designs: {}", possible_designs);
}

fn count_combinations(towels: &Vec<String>, design: &String) -> u64 {
    let design_len = design.len();
    let mut dp = vec![0u64; design_len + 1];
    dp[design_len] = 1;

    for pos in (0..design_len).rev() {
        for towel in towels {
            if design[pos..].starts_with(towel) {
                let new_pos = pos + towel.len();
                if new_pos <= design_len {
                    dp[pos] += dp[new_pos];
                }
            }
        }
    }

    dp[0]
}
