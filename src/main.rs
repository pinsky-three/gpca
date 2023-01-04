use std::time::Instant;

use gpca::{
    ds::DynamicalSystemArrayBuilder,
    dynamics::life::LifeLikeCellularAutomatonArray,
    space::{DiscreteSpaceArray, TwoDimensional},
};

use image::{ImageBuffer, Rgb, RgbImage};
use ndarray::{ArrayBase, Dim, OwnedRepr};
use rand::{thread_rng, Rng};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

const STEPS: usize = 100;

fn main() {
    let mut rng = thread_rng();

    // let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mut space = TwoDimensional::<WIDTH, HEIGHT>::new();
    space.update_state(
        &mut |state: &mut ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>| {
            state.map_inplace(|x: &mut u32| *x = rng.gen_range(0..2) as u32);
        },
    );

    let dynamic = LifeLikeCellularAutomatonArray::new(&[3, 6, 7, 8], &[3, 4, 6, 7, 8]);

    let mut ca = DynamicalSystemArrayBuilder::new(space, dynamic).build();

    let now = Instant::now();

    for _ in 0..STEPS {
        ca.tick();
    }

    let elapsed = now.elapsed();

    println!(
        "ticks per seconds: {:.2}",
        STEPS as f64 / elapsed.as_secs_f64()
    );

    let state = ca.space().read_state();

    let img: RgbImage = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        if state[[x as usize, y as usize]] == 1 {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        }
    });

    img.save("dnn_512.png").unwrap();
}
