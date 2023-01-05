use std::time::Instant;

use gpca::{
    ds::DynamicalSystemArrayBuilder,
    dynamics::life::LifeLikeCellularAutomatonArray,
    render::ImageSpaceArrayRenderer,
    space::{DiscreteSpaceArray, TwoDimensional},
};

use ndarray::{ArrayBase, Dim, OwnedRepr};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

const WIDTH: usize = 512;
const HEIGHT: usize = 512;

const STEPS: usize = 100;

fn main() {
    let mut rng = thread_rng();

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

    let img = ca.space().render();

    let entropy: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    img.save(format!("renders/{}_{}.png", ca.name(), entropy))
        .unwrap();
}
