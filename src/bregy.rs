use std::{collections::HashMap, hash::Hash};

type HyperEdge<E> = (Box<[usize]>, E);

pub struct LocalHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
    // dynamic: LocalHyperGraphEdgeDynamic<N, E>,
}

// type LocalHyperGraphEdgeDynamic<N, E> = dyn Fn(&N, &[HyperEdge<E>], &[N]) -> N + Sync + Send;

// type LocalHyperGraphEdgeDynamic<N, E> = dyn Fn(&N, &[(&[usize], E)], &[N]) -> N + Sync + Send;

pub enum ComplexHyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    SubGraphs(Box<[Self]>),
    HyperGraph(Box<LocalHyperGraph<D, N, E>>),
}
