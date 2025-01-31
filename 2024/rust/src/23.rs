use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let content = fs::read_to_string("src/23.txt").expect("Failed to read file");

    let input: Vec<Vec<&str>> = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split("-").collect::<Vec<&str>>())
        .filter(|pair| pair.iter().all(|&s| s.len() == 2))
        .collect();

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Build the graph
    for pair in &input {
        let (a, b) = (pair[0], pair[1]);
        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    // println!("Graph: {:?}", graph);

    let mut components = Vec::new();
    let mut visited = HashSet::new();

    // Find connected components
    for (&start_key, connections) in graph.iter() {
        if !start_key.starts_with("t") {
            continue;
        }
        visited.insert(start_key);
        for &connection_1 in connections.iter() {
            if visited.contains(connection_1) {
                continue;
            }
            let nodes = &graph.get(connection_1).unwrap();
            for &connection_2 in nodes.iter() {
                if visited.contains(connection_2) {
                    continue;
                }
                let nodes2 = &graph.get(connection_2).unwrap();
                for &connection_3 in nodes2.iter() {
                    if connection_3 == start_key {
                        if !components.contains(&(start_key, connection_2, connection_1)) {
                            components.push((start_key, connection_1, connection_2));
                        }
                    }
                }
            }
        }
    }

    println!("Components: {:?} {}", components, components.len());
}
