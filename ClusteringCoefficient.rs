use std::fs::OpenOptions;
use std::io::Write;

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
    // Define the graph edges with high clustering
    let edges = vec![
        (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3), 
        (4, 5), (4, 6), (4, 7), (5, 6), (5, 7), (6, 7), 
        (8, 9), (8, 10), (8, 11), (9, 10), (9, 11), (10, 11),
        (0, 4), (1, 5), (2, 6), (3, 7), (4, 8), (5, 9), (6, 10), (7, 11),
    ];

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
   let mut file = OpenOptions::new()
       .append(true)
       .create(true)
       .open("output/ClusteringCoefficient-output.txt")
       .expect("Unable to open file");

   // Write the output to the file
   writeln!(
       &mut file,
       "Average clustering coefficient: {}",
       avg_clustering_coefficient
   )
   .expect("Unable to write to file");
}
