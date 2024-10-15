use std::hash::Hash;

use crate::spaces::local::{HyperEdge, Stateable};

pub trait LocalDynamic<N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Send + Sync,
{
    fn states(&self) -> u32;
    fn update(&self, node: &N, nodes: &[N], edges: Vec<&HyperEdge<E>>) -> N;
}
