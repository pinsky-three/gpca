use gpca::{
    ds::DynamicalSystemBuilder,
    dynamics::eca::ElementaryCellularAutomaton,
    space::{DiscreteSpace, OneDimensional},
};
use image::{ImageBuffer, Rgb, RgbImage};

const N: usize = 1000;
const M: usize = 1000;

fn main() {
    let mut img: RgbImage = ImageBuffer::new(N as u32, M as u32);

    let mut space = OneDimensional::<N>::new();
    space.update_state(&|state: &mut Vec<u32>| state[N / 2] = 1);

    let mut ca =
        DynamicalSystemBuilder::new(space, ElementaryCellularAutomaton::new_from_number(90))
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

    img.save("r28_large.png").unwrap();
}
