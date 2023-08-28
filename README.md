# General Purpose Cellular Automata

This is a computational model inspired in the concurrent computational models like cellular automata and other local based system, also this model takes inspiration from the formalism of dynamical systems and the theory of computation. This model is a generalization of the async cellular automata model, in the sense that the cells can have any number of states and the neighborhood can be any hyper-graph and updates are not necessarily synchronous.

A actual implementation of the model is the following:

```rust
use std::{collections::HashMap, hash::Hash};

pub type HyperEdge<E> = Vec<(Vec<usize>, E)>;

pub trait Interaction<E>
where
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Sized,
{
    fn interact(&self, nodes: &[Self], edges: &HyperEdge<E>) -> Self;
}

pub struct HyperGraph<const D: usize, N, E>
where
    N: Clone + Sync + Send + Hash + Eq + Interaction<E>,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    nodes: Box<[N; D]>,
    edges: HashMap<usize, HyperEdge<E>>,
    node_neighbors: HashMap<usize, Vec<usize>>,
}
```
