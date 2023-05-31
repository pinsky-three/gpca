use std::{collections::HashMap, hash::Hash};

use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

pub type HyperEdge<E> = Vec<(Vec<usize>, E)>;

pub trait Interaction<E>
where
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Sized,
{
    fn interact(&self, nodes: &[Self], edges: &HyperEdge<E>) -> Self;
}

pub struct LocalHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
    node_neighbors: HashMap<usize, Vec<usize>>,
}

impl<const D: usize, N, E> LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Copy + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn new(nodes: Box<[N; D]>, edges: HashMap<usize, HyperEdge<E>>) -> Self {
        let mut s = Self {
            nodes,
            edges,
            node_neighbors: HashMap::new(),
        };

        s.update_neighbors();

        s
    }

    pub fn update_neighbors(&mut self) {
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

    pub async fn compute_with_neighbors(&mut self) {
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let neighbors = self.node_neighbors.get(&i).unwrap().to_owned();
            let neighbor_nodes = neighbors.iter().map(|i| self.nodes[*i]).collect::<Vec<N>>();

            *node = node.interact(&neighbor_nodes, &vec![]);
        });

        self.nodes = new_nodes;
    }

    pub fn nodes(&self) -> &[N; D] {
        &self.nodes
    }
}
