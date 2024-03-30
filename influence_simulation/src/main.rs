use std::collections::{HashMap, HashSet};
// use rand::prelude::*;
use rand::Rng;

// Define a struct to represent a social network graph
struct Graph {
    nodes: HashMap<usize, HashSet<usize>>, // Node ID to set of neighboring node IDs
}

impl Graph {
    // Function to create a new empty graph
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    // Function to add an edge between two nodes
    fn add_edge(&mut self, node1: usize, node2: usize) {
        self.nodes.entry(node1).or_insert(HashSet::new()).insert(node2);
        self.nodes.entry(node2).or_insert(HashSet::new()).insert(node1);
    }

    // Function to simulate the spread of influence using Independent Cascade Model
    fn simulate_independent_cascade(&self, seed_set: &HashSet<usize>) -> HashSet<usize> {
        let mut influenced_nodes = seed_set.clone(); // Start with the seed set as influenced nodes
        let mut queue: Vec<usize> = seed_set.iter().copied().collect(); // Initialize queue with seed set

        let mut rng = rand::thread_rng(); // Initialize random number generator

        while !queue.is_empty() {
            let current_node = queue.remove(0); // Get the first node in the queue
            if let Some(neighbors) = self.nodes.get(&current_node) {
                for &neighbor in neighbors {
                    // Check if the neighbor is not already influenced and if influence spreads
                    if !influenced_nodes.contains(&neighbor) && rng.gen::<f64>() < 0.5 {
                        influenced_nodes.insert(neighbor);
                        queue.push(neighbor);
                    }
                }
            }
        }

        influenced_nodes
    }

    // Function to simulate the spread of influence using Linear Threshold Model
    fn simulate_linear_threshold(&self, seed_set: &HashSet<usize>) -> HashSet<usize> {
        let mut influenced_nodes = seed_set.clone(); // Start with the seed set as influenced nodes
        let mut active_nodes = seed_set.clone(); // Start with the seed set as active nodes

        let mut rng = rand::thread_rng(); // Initialize random number generator

        while !active_nodes.is_empty() {
            let current_node = *active_nodes.iter().next().unwrap(); // Get an active node
            active_nodes.remove(&current_node);

            if let Some(neighbors) = self.nodes.get(&current_node) {
                let mut total_weight = 0;
                let mut influenced_weight = 0;

                for &neighbor in neighbors {
                    total_weight += 1;
                    if influenced_nodes.contains(&neighbor) {
                        influenced_weight += 1;
                    }
                }

                let threshold = rng.gen::<f64>(); // Generate a random threshold

                // If the influenced weight is greater than or equal to the threshold, activate the node
                if (influenced_weight as f64) / (total_weight as f64) >= threshold {
                    influenced_nodes.insert(current_node);

                    // Add neighbors to the active set if not already influenced
                    for &neighbor in neighbors {
                        if !influenced_nodes.contains(&neighbor) {
                            active_nodes.insert(neighbor);
                        }
                    }
                }
            }
        }

        influenced_nodes
    }
}

fn main() {
    let mut graph = Graph::new();

    // Add edges to create the desired connections
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(2, 4);
    graph.add_edge(3, 4);
    graph.add_edge(3, 5);
    graph.add_edge(4, 5);
    graph.add_edge(4, 6);
    graph.add_edge(5, 6);

    let seed_set: HashSet<usize> = vec![1].into_iter().collect(); // Define the seed set

    // Simulate influence spread using Independent Cascade Model
    let influenced_nodes_independent_cascade = graph.simulate_independent_cascade(&seed_set);
    println!("Influenced nodes using Independent Cascade Model: {:?}", influenced_nodes_independent_cascade);

    // Simulate influence spread using Linear Threshold Model
    let influenced_nodes_linear_threshold = graph.simulate_linear_threshold(&seed_set);
    println!("Influenced nodes using Linear Threshold Model: {:?}", influenced_nodes_linear_threshold);
}
