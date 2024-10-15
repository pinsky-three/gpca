use super::basic::HyperGraphHeap;
use crate::spaces::local::Stateable;
use std::{collections::HashMap, hash::Hash};

impl<N, E> HyperGraphHeap<N, E, (u32, u32)>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Default,
{
    pub fn new_grid(
        nodes: &Vec<N>,
        width: u32,
        height: u32,
        default_edge: E,
    ) -> HyperGraphHeap<N, E, (u32, u32)> {
        let mut edges = HashMap::<usize, Vec<(Vec<usize>, E)>>::new();

        for i in 0..(width as i32) {
            for j in 0..(height as i32) {
                let index = i * (width as i32) + j;
                let mut local_neighborhood = Vec::<(Vec<usize>, E)>::new();

                for x in [-1, 0, 1] {
                    for y in [-1, 0, 1] {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        let dx = i + x;
                        let dx = if dx < 0 {
                            width as i32 - 1
                        } else if dx >= width as i32 {
                            0
                        } else {
                            dx
                        };

                        let dy = j + y;
                        let dy = if dy < 0 {
                            height as i32 - 1
                        } else if dy >= height as i32 {
                            0
                        } else {
                            dy
                        };

                        let neighbor_index = dx * width as i32 + dy;

                        local_neighborhood
                            .push((vec![neighbor_index as usize], default_edge.clone()));
                    }
                }

                edges.insert(index as usize, local_neighborhood);
            }
        }

        HyperGraphHeap::from_nodes_and_edges(nodes.to_owned(), edges, &(width, height))
    }
}
