from typing import Iterator, List


class Edge:
    def __init__(self, a: int, b: int, w: int) -> None:
        self.a, self.b, self.w = a, b, w


class Graph:
    def __init__(self, num_nodes: int, edges: List[Edge]) -> None:
        self.num_nodes, self.edges = num_nodes, edges

    @classmethod
    def from_lines(cls, lines: Iterator[str]) -> List["Graph"]:
        graphs = []
        while True:
            num_nodes, num_edges = next(lines).split()
            num_nodes, num_edges = int(num_nodes), int(num_edges)

            if num_nodes == 0 and num_edges == 0:
                break

            edges = []
            for _ in range(num_edges):
                a, b, w = next(lines).split()
                edges.append(Edge(int(a), int(b), int(w)))

            graphs.append(cls(num_nodes, edges))

        return graphs

    @classmethod
    def from_file(cls, path: str) -> List["Graph"]:
        with open(path, "r") as f:
            return cls.from_lines(f)


def main():
    graphs = Graph.from_file("input.txt")
    for graph in graphs:
        print(
            f"Number of Nodes: {graph.num_nodes}, Number of Edges: {len(graph.edges)}"
        )


if __name__ == "__main__":
    main()
