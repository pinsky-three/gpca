use std::{collections::HashMap, hash::Hash};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::spaces::local::{HyperEdge, LocalHyperGraph, Stateable};

pub struct HyperGraphHeap<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Vec<N>,
    edges: HashMap<usize, HyperEdge<E>>,
    node_neighbors: HashMap<usize, Vec<usize>>,
}

impl<N, E> LocalHyperGraph<N, E> for HyperGraphHeap<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    fn nodes(&self) -> &Vec<N> {
        &self.nodes
    }

    fn edges(&self) -> &HashMap<usize, HyperEdge<E>> {
        &self.edges
    }

    fn node_neighbors(&self) -> &HashMap<usize, Vec<usize>> {
        &self.node_neighbors
    }

    fn update_nodes(&mut self, new_nodes: Vec<N>) {
        self.nodes = new_nodes;
    }

    fn update_edges(&mut self, new_edges: HashMap<usize, HyperEdge<E>>) {
        self.edges = new_edges;
    }
}

impl<N, E> HyperGraphHeap<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn from_nodes_and_edges(
        nodes: Vec<N>,
        edges: HashMap<usize, HyperEdge<E>>,
    ) -> HyperGraphHeap<N, E> {
        let mut s = Self {
            nodes,
            edges,
            node_neighbors: HashMap::new(),
        };

        s.update_neighbors();

        s
    }

    fn update_neighbors(&mut self) {
        let edges = &self.edges;
        let mut node_neighbors = HashMap::new();

        let all_neighbors = self
            .nodes
            .par_iter()
            .enumerate()
            .map(|(i, _)| {
                let neighbors = edges.get(&i).unwrap().to_owned();

                neighbors
                    .iter()
                    .flat_map(|i| &i.0)
                    .copied()
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        all_neighbors.iter().enumerate().for_each(|(i, neighbors)| {
            node_neighbors.insert(i, neighbors.to_owned());
        });

        self.node_neighbors = node_neighbors;
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DiscreteState {
    state: u32,
}

impl Stateable for DiscreteState {
    fn state(&self) -> u32 {
        self.state
    }

    fn from_state(state: u32) -> Self {
        Self { state }
    }
}
