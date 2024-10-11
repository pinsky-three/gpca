pub mod haca_systems;

use gpca::{
    haca::local::LocalHyperGraph,
    third::wgpu::{self, accumulation, create_gpu_device, Image},
};
use haca_systems::life::LifeState;
use image::{ImageBuffer, Rgb, RgbImage};

use ::rand::{thread_rng, Rng};
use kdam::tqdm;
// use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::haca_systems::life::new_game_of_life_hyper_graph;
// use egui_macroquad::macroquad::prelude::*;

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
    const W: usize = 1024;
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

    let mut nodes = *graph.nodes();

    // let mut mem_2 = box_array![LifeState(0); WH];

    for _ in tqdm!(0..1000) {
        let mem = nodes.iter().map(|x| x.0 as f32).collect::<Vec<f32>>();

        let res = graph.compute(Image {
            data: mem,
            width: W as u32,
            height: H as u32,
        });

        let res_data_len = res.data.len();

        nodes.iter_mut().enumerate().for_each(|(i, x)| {
            *x = LifeState(res.data[i % res_data_len] as u8);
        });

        graph.update_nodes(Box::new(nodes));
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
}

trait LatticeComputable {
    fn compute(&self, input: Image) -> Image;
}

impl<const D: usize> LatticeComputable for LocalHyperGraph<D, LifeState, ()> {
    fn compute(&self, input: Image) -> Image {
        process_wgpu::<D>(input)
    }
}

fn process_wgpu<const D: usize>(input: Image) -> Image {
    let kernel = accumulation();

    // println!("kernel.size: {:?}", kernel.size);

    let device = create_gpu_device();
    let output = futures::executor::block_on(wgpu::run(&device, &input, &kernel));

    // println!(
    //     "output: {:?}",
    //     output.data.iter().take(200).collect::<Vec<&f32>>()
    // );

    // output.data.iter_mut().for_each(|x| {
    //     *x *= 255.0;
    // });

    // output.save("output.png");

    output
}
