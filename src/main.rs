pub mod haca_systems;

use std::collections::HashMap;

use gpca::haca::haca::HyperGraph;
use image::{ImageBuffer, Rgb, RgbImage};

use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();

            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];

            unsafe { Box::from_raw(ptr) }
        }

        vec_to_boxed_array(vec![$val; $len])
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LifeState {
    Dead,
    Alive,
}

fn new_game_of_life_hyper_graph<const D: usize>(
    nodes: Box<[LifeState; D]>,
) -> HyperGraph<D, LifeState, ()> {
    let n = f32::sqrt(nodes.len() as f32) as i32;

    let mut neighbors = HashMap::<usize, Vec<(usize, ())>>::new();

    for i in 0..n {
        for j in 0..n {
            let index = i * n + j;
            let mut local_neighborhood = Vec::<(usize, ())>::new();

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

                    local_neighborhood.push((neighbor_index as usize, ()));
                }
            }

            neighbors.insert(index as usize, local_neighborhood);
        }
    }
    HyperGraph::new(nodes, neighbors)
}

#[tokio::main]
async fn main() {
    // const W: usize = 512;
    // const H: usize = W;

    // let mut mem = box_array![LifeState::Dead; 262144];

    // mem.par_iter_mut().for_each(|x| {
    //     *x = if thread_rng().gen_bool(0.5) {
    //         LifeState::Alive
    //     } else {
    //         LifeState::Dead
    //     }
    // });

    // let mut graph = new_game_of_life_hyper_graph(mem);

    // // for _ in tqdm!(0..1000) {
    // //     graph
    // //         .classic_update_nodes_by_neighborhood(|current, neighborhood| {
    // //             let neighbors = neighborhood
    // //                 .iter()
    // //                 .filter(|x| **x == LifeState::Alive)
    // //                 .count();
    // //             if neighbors == 3 {
    // //                 LifeState::Alive
    // //             } else if neighbors == 2 {
    // //                 *current
    // //             } else {
    // //                 LifeState::Dead
    // //             }
    // //         })
    // //         .await;
    // // }

    // graph
    //     .run_hashlife(1000, &|current, neighborhood| {
    //         let neighbors = neighborhood
    //             .iter()
    //             .filter(|x| **x == LifeState::Alive)
    //             .count();
    //         if neighbors == 3 {
    //             LifeState::Alive
    //         } else if neighbors == 2 {
    //             *current
    //         } else {
    //             LifeState::Dead
    //         }
    //     })
    //     .await;

    // let mut img: RgbImage = ImageBuffer::new(W as u32, H as u32);

    // let copy_mem = graph.nodes();

    // for y in 0..H {
    //     for x in 0..W {
    //         let index = (y * H) + x;

    //         if copy_mem[index] == LifeState::Alive {
    //             img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
    //         } else {
    //             img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
    //         }
    //     }
    // }

    // img.save("hca_game_of_life_test.png").unwrap();
    // let quadtree = build_quadtree_3(
    //     3,
    //     Cell {
    //         x: 0,
    //         y: 0,
    //         width: 1000,
    //         height: 1000,
    //     },
    // );

    // println!("{:?}", quadtree);

    // draw_ascii(&quadtree, 8);
}
