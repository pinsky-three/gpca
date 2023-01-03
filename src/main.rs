use gpca::{
    ds::DynamicalSystemBuilder,
    dynamics::eca::ElementaryCellularAutomaton,
    space::{DiscreteSpace, OneDimensional},
};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::{thread_rng, Rng};

const N: usize = 8000;
const M: usize = 8000;

fn main() {
    let mut rng = thread_rng();

    let mut img: RgbImage = ImageBuffer::new(N as u32, M as u32);

    let mut space = OneDimensional::<N>::new();
    space.update_state(&mut |state: &mut Vec<u32>| {
        state.iter_mut().for_each(|x| *x = rng.gen_range(0..2));
    });

    let mut ca =
        DynamicalSystemBuilder::new(space, ElementaryCellularAutomaton::new_from_number(110))
            .build();

    for y in 0..M {
        let state = ca.space().read_state();

        for x in 0..M {
            if state[x] == 1 {
                img.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
            } else {
                img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
            }
        }

        ca.tick();
    }

    img.save("r110_large.png").unwrap();
}
