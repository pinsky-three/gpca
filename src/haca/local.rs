use std::{collections::HashMap, hash::Hash};

use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

pub type HyperEdge<E> = (Vec<usize>, E);

// #[derive(Clone)]
pub struct LocalHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
}

impl<const D: usize, N, E> LocalHyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Copy + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn new(nodes: Box<[N; D]>, edges: HashMap<usize, HyperEdge<E>>) -> Self {
        Self { nodes, edges }
    }

    pub async fn compute(&mut self) {
        let edges = &self.edges;
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let neighbors = edges.get(&i).unwrap().to_owned();

            let neighbor_nodes = neighbors
                .0
                .iter()
                // .map(|i| &i.0)
                // .flatten()
                .map(|i| self.nodes[*i])
                .collect::<Vec<N>>();

            *node = node.interact(&neighbor_nodes, &neighbors);
        });

        // self.nodes = new_nodes.into_boxed_slice();
        self.nodes = new_nodes;
    }

    pub fn nodes(&self) -> &[N; D] {
        &self.nodes
    }
}

pub trait Interaction<E>
where
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Sized,
{
    fn interact(&self, nodes: &Vec<Self>, edges: &HyperEdge<E>) -> Self;
}
