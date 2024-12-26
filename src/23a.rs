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

    println!("Graph: {:?}", graph);

    let largest_clique = find_largest_clique(&graph);
    println!("Largest clique: {:?}", largest_clique);

    // sort largest clique alphabetically
    let mut largest_clique_vec: Vec<&str> = largest_clique.into_iter().collect();
    largest_clique_vec.sort_unstable();
    println!(
        "Largest clique sorted alphabetically: {}",
        largest_clique_vec.join(",")
    );
}

/// Finds the largest clique in a given graph using the Bron-Kerbosch algorithm.
///
/// # Parameters
///
/// * `graph`: A reference to a HashMap representing the graph. The keys are node names (strings),
/// and the values are HashSets of neighboring node names.
///
/// # Returns
///
/// A HashSet containing the names of nodes that form the largest clique in the graph.
fn find_largest_clique<'a>(graph: &'a HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
    let mut max_clique = HashSet::new();
    let mut potential_clique = HashSet::new();
    let mut candidates: HashSet<&str> = graph.keys().cloned().collect();
    let mut already_found = HashSet::new();

    bron_kerbosch(
        &graph,
        &mut potential_clique,
        &mut candidates,
        &mut already_found,
        &mut max_clique,
    );
    max_clique
}

/// Performs the Bron-Kerbosch algorithm to find the largest clique in a given graph.
///
/// The Bron-Kerbosch algorithm is a recursive algorithm for finding all maximal cliques in an undirected graph.
/// It operates by maintaining three sets:
/// - `potential_clique`: Nodes that are part of the current clique being explored.
/// - `candidates`: Nodes that could potentially extend the current clique.
/// - `already_found`: Nodes that have already been included in a larger clique.
///
/// The algorithm recursively explores all possible combinations of nodes to find the largest clique.
///
/// # Parameters
///
/// * `graph`: A reference to a HashMap representing the graph. The keys are node names (strings),
/// and the values are HashSets of neighboring node names.
/// * `potential_clique`: A mutable reference to a HashSet containing the nodes that are part of the current clique being explored.
/// * `candidates`: A mutable reference to a HashSet containing the nodes that could potentially extend the current clique.
/// * `already_found`: A mutable reference to a HashSet containing the nodes that have already been included in a larger clique.
/// * `max_clique`: A mutable reference to a HashSet that will store the largest clique found so far.
fn bron_kerbosch<'a>(
    graph: &'a HashMap<&'a str, HashSet<&'a str>>,
    potential_clique: &mut HashSet<&'a str>,
    candidates: &mut HashSet<&'a str>,
    already_found: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    if candidates.is_empty() && already_found.is_empty() {
        if potential_clique.len() > max_clique.len() {
            *max_clique = potential_clique.clone();
        }
        return;
    }

    let candidates_clone = candidates.clone();
    for candidate in candidates_clone.iter() {
        potential_clique.insert(candidate);
        let mut new_candidates = candidates
            .intersection(&graph[candidate])
            .cloned()
            .collect();
        let mut new_already_found = already_found
            .intersection(&graph[candidate])
            .cloned()
            .collect();
        bron_kerbosch(
            graph,
            potential_clique,
            &mut new_candidates,
            &mut new_already_found,
            max_clique,
        );
        potential_clique.remove(candidate);
        candidates.remove(candidate);
        already_found.insert(candidate);
    }
}
