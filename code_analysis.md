# Code Analysis

- E is the number of edges in the graph
- V is the number of vertices in the graph
- d is the maximum degree in the graph
- n is the number of nodes in the graph
- k is the number of neighbors of the node

## AveragePathLen.rs

### Time Complexity Analysis:

1. build_graph:
For each edge, we insert its endpoints into the adjacency list, which takes constant time on average.
<br>```Time Complexity: O(E)```

2. bfs:
BFS traverses each vertex and each edge once.
<br>```Time Complexity: O(V + E)```

3. average_path_length:
BFS (bfs function) is called for each node in the graph.
<br>```Time Complexity: O(V * (V + E))```

The most significant time complexity is O(V * (V + E)), dominated by the calculation of the average path length. Thus, the overall time complexity remains ```O(V * (V + E))```.

## ClusteringCoefficient.rs

### Time Complexity Analysis:

1. Building the Adjacency List: We iterate over each edge once to populate the adjacency list.
<br>```Time Complexity: O(E)```

2. clustering_coefficient function: We have nested loops iterating over the node's neighbors to check for triangles.
<br>```Time Complexity: O(k^2)```

3. average_clustering_coefficient: We iterate over each node in the graph, and for each node, we calculate the clustering coefficient, which has a time complexity of O(k^2).
<br>```Time Complexity: O(V * k^2)```

The most significant time complexity is O(V * k^2), dominated by the calculation of the average clustering coefficient. Thus, the overall time complexity remains ```O(V * k^2)```.

## DegreeDistribution.ipynb

### Time Complexity Analysis:

1. nx.karate_club_graph(): NetworkX typically uses adjacency list representation, and loading the graph involves iterating over all nodes and edges to construct the adjacency list.
<br>```Time Complexity: O(n + E)```

2. G.degree(): NetworkX typically stores the degree of each node, and retrieving the degree of each node involves accessing a data structure with constant time complexity.
<br>```Time Complexity: O(n)```

3. nx.degree_histogram(G): NetworkX internally counts the occurrences of each degree, which takes linear time with respect to the maximum degree.
<br>```Time Complexity: O(d)```

4. degree_probability: Calculating the probability of each degree involves iterating over the degree histogram, which has length equal to the maximum degree.
<br>```Time Complexity: O(d)```

5. plt.bar(): Plotting each bar involves iterating over the degree probability list.
<br>```Time Complexity: O(d)```

```The overall time complexity is O(n + E)```. This is because the other operations, such as calculating degree distribution and plotting the degree probability distribution, typically have time complexities that are dominated by the size of the graph, which is O(n + E).

## influence_simulation

### Time Complexity Analysis:

1. add_edge: HashMap insertion and HashSet insertion both have an average time complexity of O(1).
<br>```Time Complexity: O(1)```

2. simulate_independent_cascade: In the worst case, each node is visited once, and each edge is traversed once.
<br>```Time Complexity: O(V + E)```

3. simulate_linear_threshold: In the worst case, for each active node, we iterate through its neighbors to calculate the influenced weight.
<br>```Time Complexity: O(V * E)```


The time complexity varies depending on the operation:
- Adding edges to the graph has a time complexity of O(1) amortized.
- Simulating influence spread using the Independent Cascade Model has a time complexity of O(V + E).
- Simulating influence spread using the Linear Threshold Model has a time complexity of O(V * E).

