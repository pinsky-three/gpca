use itertools::Itertools;
use rayon::{
    prelude::{
        IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator,
        ParallelIterator,
    },
    slice::ParallelSliceMut,
};
use std::{collections::HashMap, marker::PhantomData, time::Duration, usize};

pub struct HyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send,
    E: Clone + Sync + Send,
{
    nodes: Box<[N; D]>,
    edges: Vec<Vec<usize>>,
    // edge_data: Vec<E>,
    node_neighbors: HashMap<usize, Vec<usize>>,
    edge_type: PhantomData<E>,
}

impl<const D: usize, N, E> HyperGraph<D, N, E>
where
    N: Clone + Sync + Send,
    E: Clone + Sync + Send,
{
    pub fn new(
        nodes: Box<[N; D]>,
        edges: Vec<Vec<usize>>,
        node_neighbors: HashMap<usize, Vec<usize>>,
        // edge_data_generator: fn(&Vec<usize>) -> E,
    ) -> Self {
        // let edge_data = edges.iter().map(edge_data_generator).collect();

        Self {
            nodes,
            edges,
            // edge_data,
            node_neighbors,
            edge_type: PhantomData,
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
        // let neighbors = self
        //     .edges
        //     .iter()
        //     // .enumerate()
        //     .filter(|edge| edge.contains(&node));

        // let is_node_self_referenced = neighbors
        //     .clone()
        //     .filter(|edge| edge.contains(&node))
        //     .filter(|edge| are_all_elements_equal(edge).is_some())
        //     .count()
        //     > 0;

        // neighbors
        //     .map(|edge| edge.iter().map(|e| e.to_owned()))
        //     .flatten()
        //     .unique()
        //     .sorted()
        //     .filter(|e| {
        //         if is_node_self_referenced {
        //             true
        //         } else {
        //             *e != node
        //         }
        //     })
        //     .collect()
    }

    pub async fn update_nodes_by_neighborhood(
        &mut self,
        cpu_time: Duration,
        update: impl Fn(&N, &[N]) -> N + Sync + Send,
    ) where
        N: Clone + Sync + Send,
    {
        // self.nodes.clone().chunks_mut(1_000).for_each(|chunk| {
        //     chunk.par_iter_mut().enumerate().for_each(|(i, node)| {
        //         let nodes_indexes = self.neighbors(&i);

        //         let nodes_values = nodes_indexes
        //             .iter()
        //             .map(|i| self.nodes[*i].to_owned())
        //             .collect::<Vec<_>>();

        //         *node = f(node, &nodes_values);

        //         // self.nodes.get_many_mut(nodes_indexes);
        //         // self.nodes[i] = f(node, &nodes_values);
        //     });
        // });

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

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

// fn are_all_elements_equal<T: PartialEq>(elements: &[T]) -> Option<&T> {
//     match elements {
//         [head, tail @ ..] => tail.iter().all(|x| x == head).then(|| head),
//         [] => None,
//     }
// }

trait HyperGraphAsyncDynamic<const D: usize, N, E>
where
    N: Clone + Sync + Send,
    E: Clone + Sync + Send,
{
    fn apply(&self, graph: &HyperGraph<D, N, E>) -> HyperGraph<D, N, E>;
}
