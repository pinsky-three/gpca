use super::basic::HyperGraphHeap;
use crate::spaces::local::Stateable;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

impl<N, E> HyperGraphHeap<N, E, (u32, u32)>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Default + Debug,
{
    fn xy_to_index(width: u32, x: u32, y: u32) -> u32 {
        y * width + x
    }

    pub fn new_grid(
        nodes: &Vec<N>,
        width: u32,
        height: u32,
        default_edge: E,
    ) -> HyperGraphHeap<N, E, (u32, u32)> {
        let mut edges = HashMap::<usize, Vec<(Vec<usize>, E)>>::new();

        for current_node_x in 0..width {
            for current_node_y in 0..height {
                let index = Self::xy_to_index(width, current_node_x, current_node_y);

                let mut local_neighborhood = Vec::<(Vec<usize>, E)>::new();

                for dy in [-1, 0, 1] {
                    for dx in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let neighbor_node_x = current_node_x as i32 + dx;

                        let neighbor_node_x = if neighbor_node_x < 0 {
                            width as i32 + dx
                        } else if neighbor_node_x >= width as i32 {
                            0
                        } else {
                            neighbor_node_x
                        };

                        let neighbor_node_y = current_node_y as i32 + dy;

                        let neighbor_node_y = if neighbor_node_y < 0 {
                            height as i32 + dy
                        } else if neighbor_node_y >= height as i32 {
                            0
                        } else {
                            neighbor_node_y
                        };

                        let neighbor_index = Self::xy_to_index(
                            width,
                            neighbor_node_x as u32,
                            neighbor_node_y as u32,
                        );

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
