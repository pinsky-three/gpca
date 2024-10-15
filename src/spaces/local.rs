use std::{collections::HashMap, hash::Hash};

pub type HyperEdge<E> = Vec<(Vec<usize>, E)>;

pub trait Stateable
where
    Self: Clone + Send + Sync + Hash + Eq + PartialEq,
{
    fn state(&self) -> u32;
    fn from_state(state: u32) -> Self;
}

pub trait LocalHyperGraph<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Send + Sync,
{
    fn nodes(&self) -> &Vec<N>;
    fn edges(&self) -> &HashMap<usize, HyperEdge<E>>;
    fn node_neighbors(&self) -> &HashMap<usize, Vec<usize>>;

    fn update_nodes(&mut self, new_nodes: Vec<N>);
    fn update_edges(&mut self, new_edges: HashMap<usize, HyperEdge<E>>);
}
