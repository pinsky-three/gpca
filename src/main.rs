// use std::time::Instant;

// use gpca::{
//     ds::DynamicalSystemBuilder,
//     dynamics::{eca::ElementaryCellularAutomaton, life::LifeLikeCellularAutomaton},
//     space::{DiscreteSpace, OneDimensional, TwoDimensional},
// };
// use image::{ImageBuffer, Rgb, RgbImage};
// use rand::{thread_rng, Rng};

// const WIDTH: usize = 1050;
// const HEIGHT: usize = 1050;

// const STEPS: usize = 600;

use std::{collections::HashMap, time::Duration};

use gpca::haca::HyperGraph;
use image::{ImageBuffer, Rgb, RgbImage};
use kdam::tqdm;
use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

macro_rules! box_array {
    ($val:expr ; $len:expr) => {{
        // Use a generic function so that the pointer cast remains type-safe
        fn vec_to_boxed_array<T>(vec: Vec<T>) -> Box<[T; $len]> {
            let boxed_slice = vec.into_boxed_slice();

            let ptr = ::std::boxed::Box::into_raw(boxed_slice) as *mut [T; $len];

            unsafe { Box::from_raw(ptr) }
        }

        vec_to_boxed_array(vec![$val; $len])
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifeState {
    Dead,
    Alive,
}

fn new_game_of_life_hyper_graph<const D: usize>(
    nodes: Box<[LifeState; D]>,
) -> HyperGraph<D, LifeState, u32> {
    println!("building graph");

    //: [LifeState; D]
    // let nodes = memory;
    let mut edges = vec![];

    let n = f32::sqrt(nodes.len() as f32) as i32;

    let mut neighbors = HashMap::<usize, Vec<usize>>::new();

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

                    // if neighbor_index < 0 || neighbor_index >= nodes.len() as i32 {
                    //     continue;
                    // }

                    edges.push(vec![index as usize, neighbor_index as usize]);
                    local_neighborhood.push(neighbor_index as usize);
                }
            }
            neighbors.insert(index as usize, local_neighborhood);

            if thread_rng().gen_bool(0.5) {
                neighbors
                    .get_mut(&(index as usize))
                    .unwrap()
                    .push((index as usize + 100) % n as usize);
            }
        }
    }

    println!("builded graph");
    // edges.push(vec![6, 6]);

    // println!("{:?}", nodes);
    // println!(
    //     "{:?}",
    //     edges
    //         .iter()
    //         .filter(|e| e.contains(&6))
    //         .collect::<Vec<&Vec<usize>>>()
    // );

    HyperGraph::new(nodes, edges, neighbors)
}

#[tokio::main]
async fn main() {
    const W: usize = 256;
    const H: usize = W;

    // let nodes = [LifeState::Dead, 'b', 'c', 'd', 'e'];
    // let edges = vec![
    //     vec![0, 1],
    //     vec![1, 2, 3],
    //     vec![0, 3],
    //     vec![1, 2],
    //     vec![4, 0],
    // ];
    // println!("starting");

    // let a = Vec::<LifeState>::with_capacity((1usize << 46) - 1);
    // let a = [LifeState::Dead; (1usize << 47) - 1];
    // const SIZE: usize = (1usize << 32) - 1;
    // println!(
    //     "allocating: {} bytes",
    //     SIZE * std::mem::size_of::<LifeState>()
    // );

    // let b = box_array![LifeState::Dead; (1usize << 32) - 1];
    // let b = vec![LifeState::Dead; SIZE].into_boxed_slice();

    let mut mem = box_array![LifeState::Dead; W*H];

    // let mut rng = thread_rng();

    mem.par_iter_mut().for_each(|x| {
        *x = if thread_rng().gen_bool(0.5) {
            LifeState::Alive
        } else {
            LifeState::Dead
        }
    });

    let mut graph = new_game_of_life_hyper_graph(mem);

    // println!("{:?}", graph.neighbors(&6));

    for _ in tqdm!(0..5000) {
        graph
            .update_nodes_by_neighborhood(Duration::from_millis(10), |current, neighborhood| {
                let neighbors = neighborhood
                    .iter()
                    .filter(|x| **x == LifeState::Alive)
                    .count();
                if neighbors == 3 {
                    LifeState::Alive
                } else if neighbors == 2 {
                    *current
                } else {
                    LifeState::Dead
                }
            })
            .await;

        // println!("tick: {}", i);
    }

    let mut img: RgbImage = ImageBuffer::new(W as u32, H as u32);

    // let mut space = TwoDimensional::<WIDTH, HEIGHT>::new();
    // space.update_state(&mut |state: &mut Vec<u32>| {
    //     state.iter_mut().for_each(|x| *x = rng.gen_range(0..2));
    // });

    // let dynamic = LifeLikeCellularAutomaton::new(&[3, 6, 7, 8], &[3, 4, 6, 7, 8]);

    // let mut ca = DynamicalSystemBuilder::new(space, dynamic).build();

    // let now = Instant::now();

    // for _ in 0..STEPS {
    //     ca.tick();
    // }

    // let elapsed = now.elapsed();

    // println!(
    //     "ticks per seconds: {:.2}",
    //     STEPS as f64 / elapsed.as_secs_f64()
    // );

    // let state = ca.space().read_state();
    let copy_mem = graph.nodes();

    for y in 0..H {
        for x in 0..W {
            let index = (y * H) + x;

            if copy_mem[index] == LifeState::Alive {
                img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
            }
        }
    }

    img.save("hca_game_of_life_test.png").unwrap();
}
