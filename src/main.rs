pub mod haca_systems;

use gpca::haca::local::{Interaction, LocalHyperGraph};
use haca_systems::life::LifeState;
use image::{ImageBuffer, Rgb, RgbImage};

use ::rand::{thread_rng, Rng};
use kdam::tqdm;
// use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::haca_systems::life::new_game_of_life_hyper_graph;
use egui_macroquad::macroquad::prelude::*;

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

#[tokio::main]
async fn main() {
    const W: usize = 2048;
    const H: usize = W;

    const WH: usize = W * H;

    let mut mem = box_array![LifeState(0); WH];

    mem.par_iter_mut().for_each(|x| {
        *x = if thread_rng().gen_bool(0.5) {
            LifeState(1)
        } else {
            LifeState(0)
        }
    });

    let mut graph = new_game_of_life_hyper_graph(mem);

    for _ in tqdm!(0..1000) {
        graph.compute_with_neighbors().await;
    }

    let mut img: RgbImage = ImageBuffer::new(W as u32, H as u32);

    let copy_mem = *graph.nodes();

    for y in 0..H {
        for x in 0..W {
            let index = (y * H) + x;

            if copy_mem[index] == LifeState(1) {
                img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
            }
        }
    }

    img.save("hca_game_of_life_test.png").unwrap();
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

    // graph.compute();
}

trait LatticeComputable {
    fn compute(&self);
}

impl<const D: usize> LatticeComputable for LocalHyperGraph<D, LifeState, ()> {
    fn compute(&self) {
        let nodes = self.nodes();

        let n = f32::sqrt(nodes.len() as f32) as i32;

        let (w, h) = (n, n);

        let kernel_size = 3;

        let nodes = nodes.map(|n| n.0);

        process_wgpu(nodes, w, h, kernel_size);
    }
}

fn process_wgpu<const D: usize>(memory: [u8; D], w: i32, h: i32, kernel_size: i32) -> [u8; D] {
    let mut new_memory = [0; D];

    let kernel = [
        1, 1, 1, //
        1, 1, 1, //
        1, 1, 1, //
    ];

    for i in 0..w {
        for j in 0..h {
            let mut sum = 0;

            for x in -1..2 {
                for y in -1..2 {
                    let dx = i + x;
                    let dy = j + y;

                    if dx < 0 || dx >= w || dy < 0 || dy >= h {
                        continue;
                    }

                    let index = (dx * w + dy) as usize;

                    sum += memory[index] * kernel[((x + 1) * 3 + (y + 1)) as usize];
                }
            }

            let index = (i * w + j) as usize;

            new_memory[index] = sum;
        }
    }

    new_memory
}
