use std::fmt;

struct Graph {
    num_nodes: usize,
    neighbor_matrix: Vec<Vec<i64>>,
}

impl Graph {
    fn new(num_nodes: usize, neighbor_matrix: Vec<Vec<i64>>) -> Self {
        Self {
            num_nodes,
            neighbor_matrix,
        }
    }

    fn floyd_warshall(&self) -> Vec<Vec<i64>> {
        let n = self.num_nodes;
        let mut distance = self.neighbor_matrix.clone();

        for k in 0..n {
            for a in 0..n {
                for b in 0..n {
                    distance[a][b] = distance[a][b].min(distance[a][k] + distance[k][b]);
                }
            }
        }

        distance
    }

    fn shortest_path(&self, distance: &Vec<Vec<i64>>, u: usize, v: usize) -> ShortestPathResult {
        if distance[u][v] >= i64::MAX / 2 {
            return ShortestPathResult::Impossible;
        }

        for k in 0..self.num_nodes {
            if distance[k][k] < 0 && distance[u][k] < i64::MAX / 2 && distance[k][v] < i64::MAX / 2 {
                return ShortestPathResult::NegativeInfinity;
            }
        }

        ShortestPathResult::Distance(distance[u][v])
    }
}

enum ShortestPathResult {
    Distance(i64),
    Impossible,
    NegativeInfinity,
}

impl fmt::Display for ShortestPathResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShortestPathResult::Distance(d) => write!(f, "{}", d),
            ShortestPathResult::Impossible => write!(f, "Impossible"),
            ShortestPathResult::NegativeInfinity => write!(f, "-Infinity"),
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Graph with {} nodes", self.num_nodes)
    }
}

struct Query {
    u: usize,
    v: usize,
}

struct TestCase {
    graph: Graph,
    queries: Vec<Query>,
}

impl TestCase {
    fn new(graph: Graph, queries: Vec<Query>) -> Self {
        Self { graph, queries }
    }

    fn output_format(&self) -> String {
        let distance = self.graph.floyd_warshall();
        let mut result = String::new();
        for query in &self.queries {
            let line = match self.graph.shortest_path(&distance, query.u, query.v) {
                ShortestPathResult::Distance(d) => format!("{}", d),
                ShortestPathResult::Impossible => "Impossible".to_string(),
                ShortestPathResult::NegativeInfinity => "-Infinity".to_string(),
            };
            result.push_str(&format!("{}\n", line));
        }
        result
    }

    fn from_lines<I>(lines: &mut I) -> Option<Self>
    where
        I: Iterator<Item = String>,
    {
        let first_line = lines.next()?;
        let mut parts = first_line.split_whitespace();
        let num_nodes: usize = parts.next()?.parse().unwrap();
        let num_edges: usize = parts.next()?.parse().unwrap();
        let num_queries: usize = parts.next()?.parse().unwrap();

        if num_nodes == 0 && num_edges == 0 && num_queries == 0 {
            return None;
        }

        let mut neighbor_matrix = vec![vec![i64::MAX / 2; num_nodes]; num_nodes];
        for i in 0..num_nodes {
            neighbor_matrix[i][i] = 0;
        }

        for _ in 0..num_edges {
            let line = lines.next()?;
            let mut parts = line.split_whitespace();
            let u: usize = parts.next()?.parse().unwrap();
            let v: usize = parts.next()?.parse().unwrap();
            let w: i64 = parts.next()?.parse().unwrap();
            if w < neighbor_matrix[u][v] {
                neighbor_matrix[u][v] = w;
            }
        }
        let graph = Graph::new(num_nodes, neighbor_matrix);

        let mut queries = Vec::new();
        for _ in 0..num_queries {
            let line = lines.next()?;
            let mut parts = line.split_whitespace();
            let u: usize = parts.next()?.parse().unwrap();
            let v: usize = parts.next()?.parse().unwrap();
            queries.push(Query { u, v });
        }

        Some(TestCase::new(graph, queries))
    }
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Test Case:")?;
        write!(f, "{}", self.graph)?;
        writeln!(f, "  Queries: {:?}", self.queries.len())
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
        print!("{}\n", test_case.output_format());
    }
}
