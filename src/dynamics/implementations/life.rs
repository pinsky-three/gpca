use std::hash::Hash;

use crate::{
    dynamics::local::LocalDynamic,
    spaces::{
        implementations::basic::{DiscreteState, HyperGraphHeap},
        lattice::LatticeComputable,
        local::{HyperEdge, Stateable},
    },
    system::dynamical_system::DynamicalSystem,
};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct LifeLike {
    b_list: &'static [u32],
    s_list: &'static [u32],
}

impl LifeLike {
    pub fn new(b_list: &'static [u32], s_list: &'static [u32]) -> Self {
        Self { b_list, s_list }
    }
}

impl<N, E> LocalDynamic<N, E> for LifeLike
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Clone + Send + Sync + Hash + Eq + PartialEq,
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, node: &N, nodes: &[N], _edges: Vec<&HyperEdge<E>>) -> N {
        let total = nodes.iter().map(|n| n.state()).sum();

        if self.b_list.contains(&total) {
            let a: u32 = 1; //self.states() - 1;
            N::from_state(a)
        } else if self.s_list.contains(&total) {
            node.clone()
        } else {
            N::from_state(0)
        }
    }
}

impl<N, E> LatticeComputable<N, E>
    for DynamicalSystem<HyperGraphHeap<DiscreteState, (), (u32, u32)>, LifeLike, DiscreteState, ()>
where
    N: Clone + Sync + Send + Hash + Eq,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    fn shape(&self) -> Vec<usize> {
        let (w, h) = self.space().payload();

        vec![*w as usize, *h as usize]
    }

    fn observation_neighbors(&self) -> Vec<Vec<i32>> {
        vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![0, -1],
            vec![0, 1],
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
        ]
    }
}
