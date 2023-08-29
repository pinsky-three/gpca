# General Purpose Cellular Automata

This computational model draws inspiration from concurrent computational models, notably cellular automata and other locality-based systems. Additionally, it incorporates elements from the formalism of dynamical systems and the broader theory of computation. Distinguishing itself as an advanced version of the asynchronous cellular automata model, this system allows cells to possess an arbitrary number of states. Furthermore, the neighborhood can be represented by any hyper-graph, and updates are not strictly synchronous.

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
