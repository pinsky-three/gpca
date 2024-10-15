// use std::collections::HashMap;

// use crate::spaces::local::Interaction;

// // use crate::haca::local::{HyperEdge, Interaction, LocalHyperGraph, LocalHyperGraphHeap};

// #[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Copy)]
// pub struct LifeState(pub u8);

// impl Interaction<()> for LifeState {
//     fn interact(&self, nodes: &[LifeState], _: Vec<&HyperEdge<()>>) -> LifeState {
//         let alive_neighbors = nodes.iter().filter(|&&n| n == LifeState(1)).count();

//         if alive_neighbors == 2 {
//             *self
//         } else if alive_neighbors == 3 {
//             LifeState(1)
//         } else {
//             LifeState(0)
//         }
//     }
// }

// pub fn new_game_of_life_hyper_graph<const D: usize>(
//     nodes: Box<[LifeState; D]>,
// ) -> LocalHyperGraph<D, LifeState, ()> {
//     let n = f32::sqrt(nodes.len() as f32) as i32;

//     let mut edges = HashMap::<usize, Vec<(Vec<usize>, ())>>::new();

//     for i in 0..n {
//         for j in 0..n {
//             let index = i * n + j;
//             let mut local_neighborhood = Vec::<(Vec<usize>, ())>::new();

//             for x in [-1, 0, 1] {
//                 for y in [-1, 0, 1] {
//                     if x == 0 && y == 0 {
//                         continue;
//                     }

//                     let dx = i + x;
//                     let dx = if dx < 0 {
//                         n - 1
//                     } else if dx >= n {
//                         0
//                     } else {
//                         dx
//                     };

//                     let dy = j + y;
//                     let dy = if dy < 0 {
//                         n - 1
//                     } else if dy >= n {
//                         0
//                     } else {
//                         dy
//                     };

//                     let neighbor_index = dx * n + dy;

//                     local_neighborhood.push((vec![neighbor_index as usize], ()));
//                 }
//             }

//             edges.insert(index as usize, local_neighborhood);
//         }
//     }
//     LocalHyperGraph::new(nodes, edges)
// }

// pub fn new_game_of_life_hyper_graph_heap(
//     nodes: Vec<LifeState>,
// ) -> LocalHyperGraphHeap<LifeState, ()> {
//     let n = f32::sqrt(nodes.len() as f32) as i32;

//     let mut edges = HashMap::<usize, Vec<(Vec<usize>, ())>>::new();

//     for i in 0..n {
//         for j in 0..n {
//             let index = i * n + j;
//             let mut local_neighborhood = Vec::<(Vec<usize>, ())>::new();

//             for x in [-1, 0, 1] {
//                 for y in [-1, 0, 1] {
//                     if x == 0 && y == 0 {
//                         continue;
//                     }

//                     let dx = i + x;
//                     let dx = if dx < 0 {
//                         n - 1
//                     } else if dx >= n {
//                         0
//                     } else {
//                         dx
//                     };

//                     let dy = j + y;
//                     let dy = if dy < 0 {
//                         n - 1
//                     } else if dy >= n {
//                         0
//                     } else {
//                         dy
//                     };

//                     let neighbor_index = dx * n + dy;

//                     local_neighborhood.push((vec![neighbor_index as usize], ()));
//                 }
//             }

//             edges.insert(index as usize, local_neighborhood);
//         }
//     }

//     LocalHyperGraphHeap::new(nodes, edges)
// }

// pub fn new_game_of_life_hyper_graph_quad_tree_for_hash_life(_size: usize) {}
