use std::fmt;

struct Edge {
    a: u16,
    b: u16,
    weight: u16,
}

impl Edge {
    fn new(a: u16, b: u16, weight: u16) -> Self {
        Self { a, b, weight }
    }
}

struct Neighbor {
    node: u16,
    weight: u16,
}

struct Graph {
    num_nodes: u16,
    neighbors_list: Vec<Vec<Neighbor>>,
}

impl Graph {
    fn new(num_nodes: u16, neighbors_list: Vec<Vec<Neighbor>>) -> Self {
        Self {
            num_nodes,
            neighbors_list,
        }
    }

    fn shortest_path(&self, start_node: u16, end_node: u16) -> Option<u32> {
        // Dijkstras Algorithm
        let mut distances = vec![u32::MAX; self.num_nodes as usize];
        let mut visited = vec![false; self.num_nodes as usize];
        distances[start_node as usize] = 0;

        let closest_unvisited = |distances: &[u32], visited: &[bool]| {
            let mut result = None;
            let mut min_distance = u32::MAX;
            for i in 0..visited.len() {
                if !visited[i] && distances[i] < min_distance {
                    min_distance = distances[i];
                    result = Some(i);
                }
            }
            result
        };

        loop {
            let current = closest_unvisited(&distances, &visited)?;
            visited[current] = true;

            if current == end_node as usize {
                return Some(distances[current]);
            }

            for neighbor in &self.neighbors_list[current] {
                let new_distance = distances[current].saturating_add(neighbor.weight as u32);
                if new_distance < distances[neighbor.node as usize] {
                    distances[neighbor.node as usize] = new_distance;
                }
            }
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Graph with {} nodes:", self.num_nodes)?;
        for (node, neighbors) in self.neighbors_list.iter().enumerate() {
            if !neighbors.is_empty() {
                write!(f, "  Node {}: ", node)?;
                for (i, neighbor) in neighbors.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{} (weight: {})", neighbor.node, neighbor.weight)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

struct TestCase {
    graph: Graph,
    start_node: u16,
    queries: Vec<u16>,
}

impl TestCase {
    fn new(graph: Graph, start_node: u16, queries: Vec<u16>) -> Self {
        Self {
            graph,
            start_node,
            queries,
        }
    }

    fn from_lines<I>(lines: &mut I) -> Option<Self>
    where
        I: Iterator<Item = String>,
    {
        let first_line = lines.next()?;
        let mut parts = first_line.split_whitespace();
        let num_nodes = parts.next()?.parse().unwrap();
        let num_edges = parts.next()?.parse().unwrap();
        let num_queries = parts.next()?.parse().unwrap();
        let start_node = parts.next()?.parse().unwrap();

        let should_stop = num_nodes == 0 && num_edges == 0 && num_queries == 0 && start_node == 0;
        if should_stop {
            return None;
        }

        let mut edges = Vec::new();
        for _ in 0..num_edges {
            let line = lines.next()?;
            let mut parts = line.split_whitespace();
            let u = parts.next()?.parse().unwrap();
            let v = parts.next()?.parse().unwrap();
            let w = parts.next()?.parse().unwrap();

            let edge = Edge::new(u, v, w);
            edges.push(edge);
        }

        let mut neighbors_list: Vec<Vec<Neighbor>> = (0..num_nodes).map(|_| Vec::new()).collect();
        for edge in &edges {
            neighbors_list[edge.a as usize].push(Neighbor {
                node: edge.b,
                weight: edge.weight,
            });
        }

        let graph = Graph::new(num_nodes, neighbors_list);

        let mut queries = Vec::new();
        for _ in 0..num_queries {
            let line = lines.next()?;
            let query = line.parse().unwrap();
            queries.push(query);
        }

        Some(TestCase::new(graph, start_node, queries))
    }
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Test Case:")?;
        write!(f, "{}", self.graph)?;
        writeln!(f, "  Start node: {}", self.start_node)?;
        writeln!(f, "  Queries: {:?}", self.queries)
    }
}

fn main() {
    let content = std::fs::read_to_string("sssp_input.txt").unwrap();
    let mut lines = content.lines().map(String::from);

    let test_case = TestCase::from_lines(&mut lines).unwrap();

    for query in &test_case.queries {
        if let Some(distance) = test_case.graph.shortest_path(test_case.start_node, *query) {
            println!("{}", distance);
        }
    }
}
