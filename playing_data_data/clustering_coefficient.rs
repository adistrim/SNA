use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// Define a function to calculate the clustering coefficient for a node
fn clustering_coefficient(node: usize, adjacency_list: &Vec<Vec<usize>>) -> f64 {
    let neighbors = &adjacency_list[node];
    let k = neighbors.len();
    if k < 2 {
        return 0.0; // Return 0 if the node has less than 2 neighbors
    }

    let mut triangles = 0;
    for i in 0..k {
        for j in i+1..k {
            if adjacency_list[neighbors[i]].contains(&neighbors[j]) {
                triangles += 1;
            }
        }
    }

    // Clustering coefficient formula: 2 * number of triangles / (k * (k - 1))
    2.0 * triangles as f64 / (k * (k - 1)) as f64
}

// Define a function to calculate the average clustering coefficient of the entire graph
fn average_clustering_coefficient(adjacency_list: &Vec<Vec<usize>>) -> f64 {
    let mut total_coefficient = 0.0;
    let n = adjacency_list.len();

    // Iterate through each node in the graph
    for i in 0..n {
        total_coefficient += clustering_coefficient(i, adjacency_list);
    }

    // Average clustering coefficient formula: sum of clustering coefficients / number of nodes
    total_coefficient / n as f64
}

fn main() {
    // Read edges from the text file
    let mut edges = Vec::new();
    if let Ok(file) = File::open("facebook_combined.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut parts = line.split_whitespace();
                if let (Some(source_str), Some(target_str)) = (parts.next(), parts.next()) {
                    if let (Ok(source), Ok(target)) = (source_str.parse::<usize>(), target_str.parse::<usize>()) {
                        edges.push((source, target));
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to open the file.");
        return;
    }

    // Determine the maximum node ID
    let max_node_id = edges.iter().flat_map(|&(a, b)| vec![a, b]).max().unwrap_or(0);

    // Create an adjacency list representation of the graph
    let mut adjacency_list = vec![Vec::new(); max_node_id + 1];
    for &(a, b) in &edges {
        adjacency_list[a].push(b);
        adjacency_list[b].push(a);
    }

    // Calculate the average clustering coefficient
    let avg_clustering_coefficient = average_clustering_coefficient(&adjacency_list);

    // Print the average clustering coefficient
    println!("Average clustering coefficient: {}", avg_clustering_coefficient);

    // Open the file in append mode to add the new line
    let mut file = File::create("ClusteringCoefficient-output.txt").expect("Unable to create file");

    // Write the output to the file
    writeln!(
        &mut file,
        "Average clustering coefficient: {}",
        avg_clustering_coefficient
    )
    .expect("Unable to write to file");
}
