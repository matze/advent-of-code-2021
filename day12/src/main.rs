use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse line")
    }
}

impl Error for ParseError {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Node {
    Start,
    End,
    BigCave(String),
    SmallCave(String),
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        match value {
            "start" => Node::Start,
            "end" => Node::End,
            rest => {
                if rest.chars().all(|c| c.is_uppercase()) {
                    Node::BigCave(rest.to_string())
                } else {
                    Node::SmallCave(rest.to_string())
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Result<(Node, Node), ParseError> {
    let mut split = line.split('-');
    Ok((
        split.next().ok_or_else(|| ParseError {})?.into(),
        split.next().ok_or_else(|| ParseError {})?.into(),
    ))
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum PathNode {
    Start(usize),
    End(usize),
    SmallCave(usize),
    BigCave(usize),
}

impl PathNode {
    fn index(&self) -> usize {
        match *self {
            PathNode::Start(index) => index,
            PathNode::End(index) => index,
            PathNode::SmallCave(index) => index,
            PathNode::BigCave(index) => index,
        }
    }
}

impl Graph {
    fn new<B: BufRead>(lines: &mut Lines<B>) -> Result<Self, ParseError> {
        let mut nodes = vec![];
        let mut edges = vec![];

        for line in lines {
            let line = line.map_err(|_| ParseError {})?;
            let (n1, n2) = parse_line(&line)?;

            if !nodes.contains(&n1) {
                nodes.push(n1.clone());
            }

            if !nodes.contains(&n2) {
                nodes.push(n2.clone());
            }

            let i1 = nodes.iter().position(|node| node == &n1).unwrap();
            let i2 = nodes.iter().position(|node| node == &n2).unwrap();

            edges.push((i1, i2));
            edges.push((i2, i1));
        }

        Ok(Self { edges, nodes })
    }

    /// Find index of start node
    fn start(&self) -> usize {
        self.nodes
            .iter()
            .position(|node| matches!(node, Node::Start))
            .unwrap()
    }

    /// Find indices of adjacent nodes
    fn adjacent(&self, index: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|(a, b)| if *a == index { Some(*b) } else { None })
            .collect()
    }

    /// Get node reference for an index
    fn node(&self, index: usize) -> &Node {
        &self.nodes[index]
    }

    /// Find number of paths from start index to end Node, visiting small caves only once
    fn search_once(&self) -> usize {
        let mut candidates = vec![vec![self.start()]];
        let mut paths = vec![];

        while !candidates.is_empty() {
            let mut new_candidates = vec![];

            for candidate in &candidates {
                let last = candidate.last().unwrap();

                for index in self.adjacent(*last) {
                    match self.node(index) {
                        Node::End => {
                            let mut path = candidate.clone();
                            path.push(index);
                            paths.push(path);
                        }
                        Node::Start => {
                            // We are back, so drop this path
                            continue;
                        }
                        Node::SmallCave(_) => {
                            // Only consider if we haven't visited the small cave yet
                            if !candidate.contains(&index) {
                                let mut path = candidate.clone();
                                path.push(index);
                                new_candidates.push(path);
                            }
                        }
                        Node::BigCave(_) => {
                            let mut path = candidate.clone();
                            path.push(index);
                            new_candidates.push(path);
                        }
                    }
                }
            }

            candidates = new_candidates;
        }

        paths.iter().count()
    }

    /// Find number of paths from start index to end Node, visiting a single small cave twice
    fn search_twice(&self) -> usize {
        let mut candidates = vec![vec![PathNode::Start(self.start())]];
        let mut paths = vec![];

        while !candidates.is_empty() {
            let mut new_candidates = vec![];

            for candidate in &candidates {
                let last = candidate.last().unwrap();

                for index in self.adjacent(last.index()) {
                    match self.node(index) {
                        Node::End => {
                            let mut path = candidate.clone();
                            path.push(PathNode::End(index));
                            paths.push(path);
                        }
                        Node::Start => {
                            // We are back, so drop this path
                            continue;
                        }
                        Node::SmallCave(_) => {
                            let mut smalls = candidate
                                .iter()
                                .filter_map(|c| {
                                    if matches!(c, PathNode::SmallCave(_)) {
                                        Some(c.index())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<_>>();
                            smalls.sort_unstable();

                            let old_len = smalls.len();
                            smalls.dedup();
                            let have_visited_twice = old_len > 0 && smalls.len() == old_len - 1;

                            // Only consider if we haven't visited the small cave yet
                            if !have_visited_twice
                                || !candidate.contains(&PathNode::SmallCave(index))
                            {
                                let mut path = candidate.clone();
                                path.push(PathNode::SmallCave(index));
                                new_candidates.push(path);
                            }
                        }
                        Node::BigCave(_) => {
                            let mut path = candidate.clone();
                            path.push(PathNode::BigCave(index));
                            new_candidates.push(path);
                        }
                    }
                }
            }

            candidates = new_candidates;
        }

        paths.iter().count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open("input")?);
    let graph = Graph::new(&mut reader.lines())?;
    println!("{}", graph.search_once());
    println!("{}", graph.search_twice());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn text_examples() -> Result<(), Box<dyn std::error::Error>> {
        let cursor = Cursor::new(
            r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#,
        );

        let graph = Graph::new(&mut cursor.lines())?;
        let start = graph.start();

        assert!(matches!(*graph.node(start), Node::Start));

        let adjacent = graph.adjacent(start);
        assert_eq!(adjacent.len(), 2);
        assert_eq!(graph.search_once(), 10);
        assert_eq!(graph.search_twice(), 36);

        let cursor = Cursor::new(
            r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#,
        );

        let graph = Graph::new(&mut cursor.lines())?;
        assert_eq!(graph.search_once(), 19);
        assert_eq!(graph.search_twice(), 103);

        let cursor = Cursor::new(
            r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#,
        );

        let graph = Graph::new(&mut cursor.lines())?;
        assert_eq!(graph.search_once(), 226);
        assert_eq!(graph.search_twice(), 3509);

        Ok(())
    }
}
