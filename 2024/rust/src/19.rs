use regex::Regex;
use std::fs;

fn main() {
    let content = fs::read_to_string("src/19.txt").expect("Failed to read file");

    let mut lines = content.lines();

    let towels = lines.next().unwrap().to_string().replace(", ", "|");
    let designs: Vec<String> = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();

    let towels_regex = Regex::new(&format!("^({})+$", towels)).unwrap();

    let mut possible_designs = 0u32;

    for design in &designs {
        if towels_regex.is_match(&design) {
            possible_designs += 1;
        }
    }

    println!("Possible designs: {}", possible_designs);
}
