use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const ITERATIONS: u8 = 75;

fn main() {
    let input: Vec<u64> = fs::read_to_string("src/11.txt")
        .expect("Failed to read file")
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
        })
        .collect();

    println!("{:?}", input);

    let start = Instant::now();
    let mut cache = HashMap::new();
    let res = apply_rules_recursive(input, ITERATIONS, &mut cache);
    let duration = start.elapsed();

    println!("{}", res);
    println!("Time taken: {:?}", duration);
}

fn get_value_by_rule(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![1];
    }

    if num.to_string().len() % 2 == 0 {
        let div = 10_u64.pow((num.to_string().len() / 2) as u32);
        return vec![num / div, num % div];
    }

    match num.checked_mul(2024) {
        Some(result) => vec![result],
        None => {
            println!("Overflow detected for num: {}", num);
            vec![u64::MAX]
        }
    }
}

fn apply_rules_recursive(
    input: Vec<u64>,
    iteration: u8,
    cache: &mut HashMap<(u64, u8), usize>,
) -> usize {
    if iteration == 0 {
        return input.len();
    }

    let mut result_count = 0;
    for num in input {
        if let Some(cached_count) = cache.get(&(num, iteration)) {
            result_count += *cached_count;
        } else {
            let next_sub_array = get_value_by_rule(num);
            let result = apply_rules_recursive(next_sub_array, iteration - 1, cache);
            cache.insert((num, iteration), result);
            result_count += result;
        }
    }

    return result_count;
}
