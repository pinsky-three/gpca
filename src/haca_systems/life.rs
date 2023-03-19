use std::collections::HashMap;

use gpca::haca::local::{HyperEdge, Interaction, LocalHyperGraph};

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Copy)]
pub struct LifeState(pub u8);

impl Interaction<()> for LifeState {
    fn interact(&self, nodes: &Vec<LifeState>, _: &HyperEdge<()>) -> LifeState {
        let alive_neighbors = nodes.iter().filter(|&&n| n == LifeState(1)).count();

        if alive_neighbors == 2 {
            *self
        } else if alive_neighbors == 3 {
            LifeState(1)
        } else {
            LifeState(0)
        }
    }
}

pub fn new_game_of_life_hyper_graph<const D: usize>(
    nodes: Box<[LifeState; D]>,
) -> LocalHyperGraph<D, LifeState, ()> {
    let n = f32::sqrt(nodes.len() as f32) as i32;

    let mut neighbors = HashMap::<usize, (Vec<usize>, ())>::new();

    for i in 0..n {
        for j in 0..n {
            let index = i * n + j;
            let mut local_neighborhood = Vec::<usize>::new();

            for x in vec![-1, 0, 1] {
                for y in vec![-1, 0, 1] {
                    if x == 0 && y == 0 {
                        continue;
                    }

                    let dx = i + x;
                    let dx = if dx < 0 {
                        n - 1
                    } else if dx >= n {
                        0
                    } else {
                        dx
                    };

                    let dy = j + y;
                    let dy = if dy < 0 {
                        n - 1
                    } else if dy >= n {
                        0
                    } else {
                        dy
                    };

                    let neighbor_index = dx * n + dy;

                    local_neighborhood.push(neighbor_index as usize);
                }
            }

            neighbors.insert(index as usize, (local_neighborhood, ()));
        }
    }
    LocalHyperGraph::new(nodes, neighbors)
}
