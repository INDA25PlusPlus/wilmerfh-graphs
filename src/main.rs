use std::fmt;

struct Edge {
    a: usize,
    b: usize,
    weight: u32,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  {} -- {} : {}", self.a, self.b, self.weight)
    }
}

struct Graph {
    num_nodes: u16,
    nodes: Vec<Edge>,
}

impl Graph {
    fn new(num_nodes: u16, nodes: Vec<Edge>) -> Self {
        Graph { num_nodes, nodes }
    }

    fn from_lines<I>(lines: &mut I) -> Option<Self>
    where
        I: Iterator<Item = String>,
    {
        let first_line = lines.next()?;
        let (num_nodes, num_edges) = first_line.split_once(' ')?;
        let num_nodes = num_nodes.parse().unwrap();
        let num_edges = num_edges.parse().unwrap();

        let mut edges = Vec::new();

        for _ in 0..num_edges {
            let line = lines.next()?;
            let mut parts = line.split_whitespace();
            let a = parts.next()?.parse().unwrap();
            let b = parts.next()?.parse().unwrap();
            let weight = parts.next()?.parse().unwrap();
            edges.push(Edge { a, b, weight });
        }
        Some(Graph::new(num_nodes, edges))
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph with {} nodes and {} edges:",
            self.num_nodes,
            self.nodes.len()
        )?;
        for edge in &self.nodes {
            writeln!(f, "{}", edge)?;
        }
        Ok(())
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = content.lines().map(String::from);
    if let Some(graph) = Graph::from_lines(&mut lines) {
        println!("{}", graph);
    }
}
