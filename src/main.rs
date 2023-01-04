use std::time::Instant;

use convolutions_rs::{
    convolutions::{conv2d, ConvolutionLayer},
    Padding,
};
use gpca::{
    ds::DynamicalSystemArrayBuilder,
    dynamics::life::LifeLikeCellularAutomatonArray,
    space::{DiscreteSpace, TwoDimensional},
};
use image::{ImageBuffer, Rgb, RgbImage};
use ndarray::{Array, Array2, Array3, Array4};
use rand::{thread_rng, Rng};

const WIDTH: usize = 250;
const HEIGHT: usize = 250;

const STEPS: usize = 10;

fn main() {
    let mut rng = thread_rng();

    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mut space = TwoDimensional::<250, 250>::new();
    space.update_state(&mut |state: &mut Vec<u32>| {
        state.iter_mut().for_each(|x| *x = rng.gen_range(0..2));
    });

    let dynamic = LifeLikeCellularAutomatonArray::new(&[3], &[2, 3]);

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

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (y * WIDTH) + x;

            if state[index] == 1 {
                img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
            }
        }
    }

    img.save("gol.png").unwrap();
}
