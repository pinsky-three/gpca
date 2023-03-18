/////

use gpca::haca::local::{HyperEdge, Interaction};

/////
///
///
///
///
///
///
///
///
///
///
///
///
///

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Copy)]
pub enum LifeState {
    Alive,
    #[default]
    Dead,
}

impl Interaction<()> for LifeState {
    fn interact(&self, nodes: &Vec<LifeState>, edges: &Vec<HyperEdge<()>>) -> LifeState {
        let alive_neighbors = nodes.iter().filter(|&&n| n == LifeState::Alive).count();

        if alive_neighbors == 2 {
            *self
        } else if alive_neighbors == 3 {
            LifeState::Alive
        } else {
            LifeState::Dead
        }
    }
}
