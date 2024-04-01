use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::{OpenOptions};
use std::io::{self, Write};

// Function to represent a graph as an adjacency list
fn build_graph(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(u, v) in edges {
        graph.entry(u).or_insert_with(Vec::new).push(v);
        graph.entry(v).or_insert_with(Vec::new).push(u); // For undirected graph
    }

    graph
}

// Function to perform Breadth First Search
fn bfs(graph: &HashMap<usize, Vec<usize>>, start: usize) -> HashMap<usize, usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new(); // (node, distance)

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((node, dist)) = queue.pop_front() {
        distances.insert(node, dist);

        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    queue.push_back((neighbor, dist + 1));
                    visited.insert(neighbor);
                }
            }
        }
    }

    distances
}

// Function to calculate average path length of the graph
fn average_path_length(graph: &HashMap<usize, Vec<usize>>) -> f64 {
    let mut total_length = 0;
    let num_nodes = graph.len();

    for node in graph.keys() {
        let distances = bfs(graph, *node);

        for &length in distances.values() {
            total_length += length;
        }
    }

    // Divide total path length by number of node pairs to get average path length
    (total_length as f64) / ((num_nodes * (num_nodes - 1)) as f64)
}

fn main() -> io::Result<()> {
    // Example graph represented by its edges
    let edges = vec![
        (0, 1), (0, 2), (0, 3), (1, 4), (1, 5), (2, 5), (3, 6), (4, 7), (5, 8), (6, 8), (7, 9), (8, 9)
    ];
    let graph = build_graph(&edges);

    // Calculate the average path length
    let avg_length = average_path_length(&graph);

    // Write the average path length to a file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output/AveragePahLen-output.txt")?;
    writeln!(file, "Average Path Length: {:.2}", avg_length)?;

    println!("Average Path Length: {:.2}", avg_length);

    Ok(())
}
