use derive_more::Display;
use std::{
    borrow::Borrow,
    collections::{HashSet, LinkedList},
    error::Error,
    fmt,
    hash::Hash,
};

#[derive(Default, Debug, Display)]
#[display(fmt = "Node(id:{}, value:{})", id, value)]
pub struct Node<I, V> {
    id: I,
    value: V,
}

#[derive(Default, Debug, Display)]
#[display(fmt = "Edge(from: {}, to: {}, weight: {})", from, to, weight)]
pub struct Edge<I> {
    from: I,
    to: I,
    weight: usize,
}

#[derive(Default, Debug, Display)]
#[display(fmt = "DGraph(nodes: {:?}, edges: {:?})", nodes, edges)]
pub struct DGraph<I, V> {
    nodes: HashSet<Node<I, V>>,
    edges: HashSet<Edge<I>>,
}

impl<I, V> DGraph<I, V> {
    pub fn new() -> Self {
        DGraph {
            nodes: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

impl<I, V> Node<I, V> {
    pub fn new(id: I, value: V) -> Node<I, V> {
        Node { id, value }
    }
}

impl<I> Edge<I> {
    pub fn new(from: I, to: I, weight: usize) -> Edge<I> {
        Edge { from, to, weight }
    }
}

impl<I: PartialEq, V> PartialEq for Node<I, V> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<I: PartialEq> PartialEq for Edge<I> {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl<I: Eq, V> Eq for Node<I, V> {}
impl<I: Eq> Eq for Edge<I> {}

impl<I: Hash, V> Hash for Node<I, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<I: Hash> Hash for Edge<I> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl<I, V> Borrow<I> for Node<I, V> {
    fn borrow(&self) -> &I {
        &self.id
    }
}

pub trait Graph<I, V> {
    fn root_nodes(&self) -> HashSet<&Node<I, V>>;
    fn leaf_nodes(&self) -> HashSet<&Node<I, V>>;
    fn add_node(&mut self, node: Node<I, V>) -> Result<(), Box<dyn Error>>;
    fn add_edge(&mut self, edge: Edge<I>) -> Result<(), Box<dyn Error>>;
    fn prev_nodes(&self, node: Node<I, V>) -> Option<HashSet<&Node<I, V>>>;
    fn next_nodes(&self, node: Node<I, V>) -> Option<HashSet<&Node<I, V>>>;
    fn is_valid_edge(&self, edge: &Edge<I>) -> bool;
}

impl<I, V> Graph<I, V> for DGraph<I, V>
where
    I: Eq + Hash,
{
    fn root_nodes(&self) -> HashSet<&Node<I, V>> {
        let Self { edges, nodes } = self;
        let to_nodes = edges.iter().map(|edge| &edge.to).collect::<HashSet<&I>>();
        nodes
            .iter()
            .filter(|node| !to_nodes.contains(&node.id))
            .collect::<HashSet<&Node<I, V>>>()
    }

    fn leaf_nodes(&self) -> HashSet<&Node<I, V>> {
        let Self { edges, nodes } = self;
        let from_nodes = edges.iter().map(|edge| &edge.from).collect::<HashSet<&I>>();
        nodes
            .iter()
            .filter(|node| !from_nodes.contains(&node.id))
            .collect::<HashSet<&Node<I, V>>>()
    }

    fn add_node(&mut self, node: Node<I, V>) -> Result<(), Box<dyn Error>> {
        if self.nodes.contains(&node) {
            return Err(Box::new(GraphError));
        }

        self.nodes.insert(node);
        Ok(())
    }

    fn add_edge(&mut self, edge: Edge<I>) -> Result<(), Box<dyn Error>> {
        if self.edges.contains(&edge) || !self.is_valid_edge(&edge) {
            return Err(Box::new(GraphError));
        }

        self.edges.insert(edge);
        Ok(())
    }

    fn prev_nodes(&self, node: Node<I, V>) -> Option<HashSet<&Node<I, V>>> {
        let Self { nodes, edges } = self;
        let prev_nodes = edges
            .iter()
            .filter(|edge| edge.to == node.id)
            .map(|edge| nodes.get(&edge.from))
            .flatten()
            .collect::<HashSet<&Node<I, V>>>();

        if prev_nodes.is_empty() {
            None
        } else {
            Some(prev_nodes)
        }
    }

    fn next_nodes(&self, node: Node<I, V>) -> Option<HashSet<&Node<I, V>>> {
        let Self { nodes, edges } = self;
        let next_nodes = edges
            .iter()
            .filter(|edge| edge.from == node.id)
            .map(|edge| nodes.get(&edge.to))
            .flatten()
            .collect::<HashSet<&Node<I, V>>>();

        if next_nodes.is_empty() {
            None
        } else {
            Some(next_nodes)
        }
    }

    fn is_valid_edge(&self, new_edge: &Edge<I>) -> bool {
        if new_edge.from == new_edge.to {
            return false;
        }

        let Self { nodes, edges } = self;
        if !nodes.contains(&new_edge.from) {
            return false;
        }

        if !nodes.contains(&new_edge.to) {
            return false;
        }

        // whether has cycle.
        let from_node_id = &new_edge.from;
        let mut queue = LinkedList::new();
        queue.push_back(&new_edge.to);
        while !queue.is_empty() {
            let to_node_id = queue.pop_front();
            for edge in edges {
                if to_node_id == Some(&edge.from) {
                    if from_node_id == &edge.to {
                        return false;
                    }

                    queue.push_back(&edge.to)
                }
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct GraphError;
impl Error for GraphError {}
impl fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "graph error")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::{DGraph, Edge, Graph, Node};

    #[test]
    fn test() {
        let mut set = HashSet::new();
        set.insert(Vec::<i32>::with_capacity(10));
        assert_eq!(set.get(&[][..]).unwrap().capacity(), 10);
    }

    #[test]
    fn test_hash_set() {
        let mut map = HashMap::new();
        map.insert("k".to_string(), 2);
        println!("{}", map.get("k").unwrap())
    }

    #[test]
    fn test_graph_root_nodes() {
        let mut graph = DGraph::new();
        let _ = graph.add_node(Node::new(1, "one"));
        let _ = graph.add_node(Node::new(2, "two"));
        let _ = graph.add_node(Node::new(3, "three"));

        let _ = graph.add_edge(Edge::new(1, 2, 0));
        let _ = graph.add_edge(Edge::new(1, 3, 0));
        let _ = graph.add_edge(Edge::new(2, 3, 0));

        let invalid = graph.add_edge(Edge::new(3, 1, 0));
        assert!(matches!(invalid, Err(_)));

        println!("root nodes: {:?}", graph.root_nodes());
        println!("leaf nodes: {:?}", graph.leaf_nodes());
        println!("prev nodes: {:?}", graph.prev_nodes(Node::new(3, "three")));
        println!("next nodes: {:?}", graph.next_nodes(Node::new(2, "one")));

        println!("graph: {}", graph);
    }
}
