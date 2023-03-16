use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
    hash::Hash,
};

type HyperEdge<E> = (Box<Vec<usize>>, E);

#[derive(Clone)]
pub struct LocalHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
}

impl<const D: usize, N, E> LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn is_leaf_node(&self, node_id: usize) -> bool {
        let count = self
            .edges
            .values()
            .filter(|edge| edge.0.contains(&node_id))
            .count();
        count == 1
    }
}

#[derive(Clone)]
pub enum ComplexHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    SubGraphs(HashMap<HyperEdge<E>, Self>),
    HyperGraph(Box<LocalHyperGraph<D, N, E>>),
}

impl<const D: usize, N, E> ComplexHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn if_leaf<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&LocalHyperGraph<D, N, E>) -> R,
    {
        match self {
            ComplexHyperGraph::SubGraphs(_) => None,
            ComplexHyperGraph::HyperGraph(hg) => Some(f(hg)),
        }
    }
}

fn create_quadtree<const D: usize>(
    depth: usize,
    current_depth: usize,
) -> ComplexHyperGraph<D, Vec<bool>, ()> {
    if current_depth == depth {
        let nodes_vec: Vec<Vec<bool>> = vec![vec![false; 1 << current_depth]; D];
        let nodes: Box<[Vec<bool>; D]> = match nodes_vec.try_into() {
            Ok(array) => Box::new(array),
            Err(_) => panic!("Failed to convert Vec into Box<[Vec<bool>; D]>"),
        };
        ComplexHyperGraph::HyperGraph(Box::new(LocalHyperGraph {
            nodes,
            edges: HashMap::new(),
        }))
    } else {
        let mut subgraphs = HashMap::new();
        for _ in 0..4 {
            let subgraph = create_quadtree::<D>(depth, current_depth + 1);
            let edge = (Box::new(vec![0]), ());
            subgraphs.insert(edge, subgraph);
        }
        ComplexHyperGraph::SubGraphs(subgraphs)
    }
}

pub fn build_quadtree<const D: usize>(depth: usize) -> ComplexHyperGraph<D, Vec<bool>, ()> {
    create_quadtree::<D>(depth, 0)
}

impl<const D: usize, N, E> Debug for LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("LocalHyperGraph")
            .field("nodes", &self.nodes)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<const D: usize, N, E> Debug for ComplexHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ComplexHyperGraph::SubGraphs(subgraphs) => f
                .debug_map()
                .entries(subgraphs.iter().map(|(k, v)| (k, v)))
                .finish(),
            ComplexHyperGraph::HyperGraph(hg) => hg.fmt(f),
        }
    }
}
