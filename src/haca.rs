use dashmap::DashMap;
use kdam::tqdm;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::{collections::HashMap, hash::Hash, marker::PhantomData};

pub struct HyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send,
{
    nodes: Box<[N; D]>,
    edges: Vec<Vec<usize>>,
    node_neighbors: HashMap<usize, Vec<usize>>,
    edge_type: PhantomData<E>,
    memoization: DashMap<(N, Vec<N>), N>,
}

type HyperGraphDynamic<N> = dyn Fn(&N, &[N]) -> N + Sync + Send;

impl<const D: usize, N, E> HyperGraph<D, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send,
{
    pub fn new(
        nodes: Box<[N; D]>,
        edges: Vec<Vec<usize>>,
        node_neighbors: HashMap<usize, Vec<usize>>,
    ) -> Self {
        Self {
            nodes,
            edges,
            node_neighbors,
            edge_type: PhantomData,
            memoization: DashMap::new(),
        }
    }

    pub fn nodes(&self) -> &[N; D] {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<Vec<usize>> {
        &self.edges
    }

    pub fn neighbors(&self, node: &usize) -> Vec<usize> {
        self.node_neighbors.get(node).unwrap().to_owned()
    }

    pub async fn run_hashlife(&mut self, generations: usize, update: &HyperGraphDynamic<N>) {
        for _ in tqdm!(0..generations) {
            self.update_nodes_by_neighborhood(update).await;
        }
    }

    pub async fn update_nodes_by_neighborhood<'a>(&mut self, update: &'a HyperGraphDynamic<N>)
    where
        N: Clone + Sync + Send + Sized + Hash + Eq,
    {
        let mut new_nodes = self.nodes.clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let nodes_indexes = self.neighbors(&i);

            let nodes_values = nodes_indexes
                .iter()
                .map(|i| self.nodes[*i].to_owned())
                .collect::<Vec<N>>();

            let node_with_neighbors = (node.clone(), nodes_values.clone());

            // Check if the node with its neighbors is memoized
            if let Some(new_node) = self.memoization.get(&node_with_neighbors) {
                *node = new_node.value().clone();
            } else {
                let new_node = update(node, &nodes_values);
                self.memoization
                    .insert(node_with_neighbors, new_node.clone());
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
                .map(|i| self.nodes[*i].to_owned())
                .collect::<Vec<_>>();

            *node = update(node, &nodes_values);

            // self.nodes.get_many_mut(nodes_indexes);
            // self.nodes[i] = f(node, &nodes_values);
        });

        self.nodes = new_nodes;
    }
}
