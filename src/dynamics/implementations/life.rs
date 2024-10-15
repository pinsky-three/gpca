// use std::marker::PhantomData;

// use crate::{
//     dynamic::Dynamic,
//     space::{DiscreteSpace, TwoDimensional},
// };

// #[derive(Debug, Clone)]
// pub struct LifeLikeCellularAutomaton<S: DiscreteSpace<2>> {
//     b_list: &'static [u32],
//     s_list: &'static [u32],
//     phantom: PhantomData<S>,
// }

// impl<const X: usize, const Y: usize> LifeLikeCellularAutomaton<TwoDimensional<X, Y>> {
//     pub fn new(b_list: &'static [u32], s_list: &'static [u32]) -> Self {
//         Self {
//             b_list,
//             s_list,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<const X: usize, const Y: usize> Dynamic<2, TwoDimensional<X, Y>>
//     for LifeLikeCellularAutomaton<TwoDimensional<X, Y>>
// {
//     fn states(&self) -> u32 {
//         2
//     }

//     fn update(&self, input: &[u32]) -> Vec<u32> {
//         let mut output: Vec<u32> = input.to_vec();

//         for x in 0..X as i32 {
//             for y in 0..Y as i32 {
//                 let neighbors = {
//                     let mut n = 0;

//                     for i in [-1, 0, 1] {
//                         for j in [-1, 0, 1] {
//                             if i == 0 && j == 0 {
//                                 continue;
//                             }

//                             let x = (x + i + X as i32) % X as i32;
//                             let y = (y + j + Y as i32) % Y as i32;

//                             let current_cell = (y * X as i32) + x;

//                             n += input[current_cell as usize];
//                         }
//                     }

//                     n
//                 };

//                 let current_cell = (y * X as i32) + x;

//                 if self.b_list.contains(&neighbors) {
//                     output[current_cell as usize] = 1;
//                 } else if self.s_list.contains(&neighbors) {
//                     output[current_cell as usize] = input[current_cell as usize];
//                 } else {
//                     output[current_cell as usize] = 0;
//                     // println!("set 0");
//                 }
//             }
//         }

//         output
//     }
// }

use std::hash::Hash;

use crate::{
    dynamics::local::LocalDynamic,
    spaces::local::{HyperEdge, Stateable},
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
