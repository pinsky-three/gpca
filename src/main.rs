pub mod haca_systems;

use haca_systems::life::LifeState;
use image::{ImageBuffer, Rgb, RgbImage};

use kdam::tqdm;
use rand::{thread_rng, Rng};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator};

use crate::haca_systems::life::new_game_of_life_hyper_graph;

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

    let mut mem = box_array![LifeState(0); W * H];

    mem.par_iter_mut().for_each(|x| {
        *x = if thread_rng().gen_bool(0.5) {
            LifeState(1)
        } else {
            LifeState(0)
        }
    });

    let mut graph = new_game_of_life_hyper_graph(mem);

    for _ in tqdm!(0..1000) {
        graph.compute().await;
    }

    let mut img: RgbImage = ImageBuffer::new(W as u32, H as u32);

    let copy_mem = graph.nodes().clone();

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
}
