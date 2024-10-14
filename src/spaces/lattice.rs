use std::hash::Hash;

use super::local::{Interaction, LocalHyperGraphHeapTrait};

pub trait LatticeComputable<M, N, E>
where
    M: LocalHyperGraphHeapTrait<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    fn shape(&self) -> Vec<usize>;
    fn observation_neighbors(&self) -> Vec<Vec<i32>>;
}
