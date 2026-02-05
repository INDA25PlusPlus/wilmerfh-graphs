// Kattis submission: https://kth.kattis.com/courses/DD2016/plusplus24/assignments/j8fgxh/submissions/19101678

use std::fmt;

struct UnionFind {
    parents: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parents: (0..size).collect(),
        }
    }

    fn unite(&mut self, a: usize, b: usize) {
        let a_parent = self.find(a);
        let b_parent = self.find(b);

        if a_parent != b_parent {
            self.parents[a_parent] = b_parent;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let parent = self.parents[x];
        if parent == x { x } else { self.find(parent) }
    }
}

#[derive(Clone)]
struct Edge {
    a: usize,
    b: usize,
    weight: i32,
}

impl Edge {
    fn new(a: usize, b: usize, weight: i32) -> Self {
        let mut edge = Edge { a, b, weight };
        edge.normalize();
        edge
    }

    fn normalize(&mut self) {
        if self.b < self.a {
            (self.a, self.b) = (self.b, self.a);
        }
    }

    fn output_format(&self) -> String {
        format!("{} {}", self.a, self.b)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  {} -- {} : {}", self.a, self.b, self.weight)
    }
}

struct Graph {
    num_nodes: u16,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(num_nodes: u16, edges: Vec<Edge>) -> Self {
        let mut graph = Graph { num_nodes, edges };
        graph.sort_edges();
        graph
    }

    fn from_lines<I>(lines: &mut I) -> Option<Self>
    where
        I: Iterator<Item = String>,
    {
        let first_line = lines.next()?;
        let (num_nodes, num_edges) = first_line.split_once(' ')?;
        let num_nodes = num_nodes.parse().unwrap();
        let num_edges = num_edges.parse().unwrap();

        if num_nodes == 0 && num_edges == 0 {
            return None;
        }

        let mut edges = Vec::new();

        for _ in 0..num_edges {
            let line = lines.next()?;
            let mut parts = line.split_whitespace();
            let a: usize = parts.next()?.parse().unwrap();
            let b: usize = parts.next()?.parse().unwrap();
            let weight: i32 = parts.next()?.parse().unwrap();
            edges.push(Edge::new(a, b, weight));
        }
        Some(Graph::new(num_nodes, edges))
    }

    fn sort_edges(&mut self) {
        self.edges.sort_by(|a, b| a.weight.cmp(&b.weight));
    }

    fn mst(&self) -> Option<Self> {
        // Kruskal's algorithm
        let mut edges = Vec::new();
        let mut uf = UnionFind::new(self.num_nodes as usize);

        for edge in &self.edges {
            if uf.find(edge.a) == uf.find(edge.b) {
                continue;
            }

            edges.push(edge.clone());
            uf.unite(edge.a, edge.b);
        }

        let is_valid = edges.len() == (self.num_nodes - 1) as usize;
        if is_valid {
            Some(Graph {
                num_nodes: self.num_nodes,
                edges,
            })
        } else {
            None
        }
    }

    fn total_cost(&self) -> i32 {
        self.edges.iter().map(|e| e.weight).sum()
    }

    fn output_format(&self) -> String {
        let mut sorted_edges = self.edges.clone();
        sorted_edges.sort_by_key(|e| (e.a, e.b));

        let mut result = format!("{}\n", self.total_cost());
        for edge in sorted_edges {
            result.push_str(&edge.output_format());
            result.push('\n');
        }
        result
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Graph with {} nodes and {} edges:",
            self.num_nodes,
            self.edges.len()
        )?;
        for edge in &self.edges {
            writeln!(f, "{}", edge)?;
        }
        Ok(())
    }
}

fn main() {
    let content = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut lines = content.lines().map(String::from);
    let mut graphs = Vec::new();

    loop {
        match Graph::from_lines(&mut lines) {
            Some(graph) => graphs.push(graph),
            None => break,
        }
    }

    for graph in graphs {
        match graph.mst() {
            Some(mst) => print!("{}", mst.output_format()),
            None => println!("Impossible"),
        }
    }
}
