use std::hash::Hash;

// use super::local::{Interaction, LocalHyperGraphHeapTrait};

pub trait LatticeComputable<N, E>
where
    // M: LocalHyperGraphHeapTrait<N, E>,
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    fn shape(&self) -> Vec<usize>;
    fn observation_neighbors(&self) -> Vec<Vec<i32>>;
}
