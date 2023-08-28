use dashmap::DashMap;
use itertools::Itertools;
use kdam::tqdm;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::{collections::HashMap, hash::Hash};

pub struct HyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    node_neighbors: HashMap<usize, Vec<(usize, E)>>,
    memoization: DashMap<(N, Vec<N>), N>,
}

type HyperGraphDynamic<N> = dyn Fn(&N, &[N]) -> N + Sync + Send;

impl<const D: usize, N, E> HyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Copy,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn new(nodes: Box<[N; D]>, node_neighbors: HashMap<usize, Vec<(usize, E)>>) -> Self {
        Self {
            nodes,
            node_neighbors,
            memoization: DashMap::new(),
        }
    }

    pub fn nodes(&self) -> &[N; D] {
        &self.nodes
    }

    pub fn edges(&self) -> Vec<E> {
        self.node_neighbors
            .values()
            .flatten()
            .map(|(_, e)| e)
            .unique()
            .map(|e| e.to_owned())
            .collect::<Vec<E>>()
    }

    pub fn neighbors(&self, node: &usize) -> Vec<(usize, E)> {
        self.node_neighbors.get(node).unwrap().to_owned()
    }

    pub async fn run_hash_life(&mut self, generations: usize, update: &HyperGraphDynamic<N>) {
        for _ in tqdm!(0..generations) {
            self.update_nodes_by_neighborhood(update).await;
        }
    }

    pub async fn update_nodes_by_neighborhood<'a>(&mut self, update: &'a HyperGraphDynamic<N>)
    where
        N: Clone + Sync + Send + Sized + Hash + Eq + Copy,
    {
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let nodes_indexes = self.neighbors(&i);

            let nodes_values = nodes_indexes
                .iter()
                .map(|i| self.nodes[i.0].to_owned())
                .collect::<Vec<N>>();

            let node_with_neighbors = (*node, nodes_values.clone());

            // Check if the node with its neighbors is memoized
            if let Some(new_node) = self.memoization.get(&node_with_neighbors) {
                *node = *new_node.value();
            } else {
                let new_node = update(node, &nodes_values);
                self.memoization.insert(node_with_neighbors, new_node);
                *node = new_node;
            }
        });

        self.nodes = new_nodes;
    }

    pub async fn classic_update_nodes_by_neighborhood(
        &mut self,
        update: impl Fn(&N, &[N]) -> N + Sync + Send,
    ) where
        N: Clone + Sync + Send,
    {
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let nodes_indexes = self.neighbors(&i);

            let nodes_values = nodes_indexes
                .iter()
                .map(|i| self.nodes[i.0].to_owned())
                .collect::<Vec<_>>();

            *node = update(node, &nodes_values);
        });

        self.nodes = new_nodes;
    }
}
