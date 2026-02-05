use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
        let mut heap = BinaryHeap::new();

        distances[start_node as usize] = 0;
        heap.push(Reverse((0u32, start_node)));

        while let Some(Reverse((dist, node))) = heap.pop() {
            if node == end_node {
                return Some(dist);
            }

            if dist > distances[node as usize] {
                continue;
            }

            for neighbor in &self.neighbors_list[node as usize] {
                let new_distance = dist.saturating_add(neighbor.weight as u32);
                if new_distance < distances[neighbor.node as usize] {
                    distances[neighbor.node as usize] = new_distance;
                    heap.push(Reverse((new_distance, neighbor.node)));
                }
            }
        }

        None
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

impl TestCase {
    fn output_format(&self) -> String {
        let mut result = String::new();
        for query in &self.queries {
            match self.graph.shortest_path(self.start_node, *query) {
                Some(distance) => result.push_str(&format!("{}\n", distance)),
                None => result.push_str("Impossible\n"),
            }
        }
        result
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
    let content = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut lines = content.lines().map(String::from);
    let mut test_cases = Vec::new();

    loop {
        match TestCase::from_lines(&mut lines) {
            Some(test_case) => test_cases.push(test_case),
            None => break,
        }
    }

    for test_case in test_cases {
        print!("{}", test_case.output_format());
    }
}
