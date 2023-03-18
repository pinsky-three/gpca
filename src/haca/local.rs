use std::{collections::HashMap, hash::Hash};

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

pub type HyperEdge<E> = (Box<Vec<usize>>, E);

#[derive(Clone)]
pub struct LocalHyperGraph<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Vec<N>,
    edges: HashMap<usize, Vec<HyperEdge<E>>>,
}

impl<N, E> LocalHyperGraph<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Default + Copy + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn new(nodes: Vec<N>, edges: HashMap<usize, Vec<HyperEdge<E>>>) -> Self {
        Self { nodes, edges }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: vec![N::default(); capacity],
            edges: HashMap::with_capacity(capacity),
        }
    }

    pub async fn classic_update_nodes_by_neighborhood(&mut self) {
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let neighbors = self.edges.get(&i).unwrap().to_owned();

            let nodes = neighbors
                .iter()
                .map(|i| (*i).0.to_vec())
                .flatten()
                .map(|i| self.nodes[i].to_owned())
                .collect::<Vec<_>>();

            *node = node.interact(&nodes, &neighbors);

            // *node = update(node, &nodes_values);
        });

        // self.nodes = new_nodes;
    }
}

pub trait Interaction<E>
where
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Sized,
{
    fn interact(&self, nodes: &Vec<Self>, edges: &Vec<HyperEdge<E>>) -> Self;
}
