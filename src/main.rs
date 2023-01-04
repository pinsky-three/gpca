use std::time::Instant;

use gpca::{
    ds::DynamicalSystemBuilder,
    dynamics::{eca::ElementaryCellularAutomaton, life::LifeLikeCellularAutomaton},
    space::{DiscreteSpace, OneDimensional, TwoDimensional},
};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};

const WIDTH: usize = 1050;
const HEIGHT: usize = 1050;

const STEPS: usize = 600;

fn main() {
    let mut rng = thread_rng();

    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let mut space = TwoDimensional::<WIDTH, HEIGHT>::new();
    space.update_state(&mut |state: &mut Vec<u32>| {
        state.iter_mut().for_each(|x| *x = rng.gen_range(0..2));
    });

    let dynamic = LifeLikeCellularAutomaton::new(&[3, 6, 7, 8], &[3, 4, 6, 7, 8]);

    let mut ca = DynamicalSystemBuilder::new(space, dynamic).build();

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

    img.save("day_night_2.png").unwrap();
}
