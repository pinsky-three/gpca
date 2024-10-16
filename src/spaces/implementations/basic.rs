use std::{collections::HashMap, hash::Hash};

use rand::{rngs::ThreadRng, Rng};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

use crate::spaces::local::{HyperEdge, LocalHyperGraph, Stateable};

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct DiscreteState {
    state: u32,
}

impl DiscreteState {
    pub fn filled_vector(size: u32, states: u32) -> Vec<DiscreteState> {
        // vec![DiscreteState { state }; size]
        (0..size)
            .into_par_iter()
            .map(|_i| DiscreteState::from_state(ThreadRng::default().gen_range(0..states)))
            .collect::<Vec<DiscreteState>>()
    }
}

impl Stateable for DiscreteState {
    fn state(&self) -> u32 {
        self.state
    }

    fn from_state(state: u32) -> Self {
        Self { state }
    }
}

pub struct HyperGraphHeap<N, E, P>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Vec<N>,
    edges: HashMap<usize, HyperEdge<E>>,
    node_neighbors: HashMap<usize, Vec<usize>>,

    payload: P,
}

impl<N, E, P> LocalHyperGraph<N, E> for HyperGraphHeap<N, E, P>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    P: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
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

impl<N, E, P> HyperGraphHeap<N, E, P>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    P: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn payload(&self) -> &P {
        &self.payload
    }
}

impl<N, E, P> HyperGraphHeap<N, E, P>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    P: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Default,
{
    pub fn from_nodes_and_edges(
        nodes: Vec<N>,
        edges: HashMap<usize, HyperEdge<E>>,
        payload: &P,
    ) -> HyperGraphHeap<N, E, P> {
        let mut s = Self {
            nodes,
            edges,
            node_neighbors: HashMap::new(),
            payload: payload.clone(),
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
                if let Some(neighbors) = edges.get(&i) {
                    neighbors
                        .iter()
                        .flat_map(|i| &i.0)
                        .copied()
                        .collect::<Vec<usize>>()
                } else {
                    vec![]
                }
                // let neighbors = edges.get(&i).unwrap().to_owned();

                // neighbors
                //     .iter()
                //     .flat_map(|i| &i.0)
                //     .copied()
                //     .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        all_neighbors.iter().enumerate().for_each(|(i, neighbors)| {
            node_neighbors.insert(i, neighbors.to_owned());
        });

        self.node_neighbors = node_neighbors;
    }
}
