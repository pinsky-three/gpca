use gpca::{
    dynamics::implementations::life::LifeLike,
    spaces::{
        implementations::basic::{DiscreteState, HyperGraphHeap},
        local::Stateable,
    },
    system::dynamical_system::DynamicalSystem,
    third::wgpu::create_gpu_device,
};
use image::{Rgb, RgbImage};
use kdam::tqdm;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[tokio::main]
async fn main() {
    const W: u32 = 512;
    const H: u32 = W;

    const STATES: u32 = 2;

    let device = create_gpu_device();

    let space = HyperGraphHeap::new_grid(&DiscreteState::filled_vector(W * H, STATES), W, H, ());

    let dynamic = LifeLike::new(&[3], &[2, 3]);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    for _ in tqdm!(0..10) {
        system.compute_sync_wgpu(&device);
    }

    let states = system
        .space_state()
        .par_iter()
        .map(|x| x.state() as u8)
        .collect::<Vec<u8>>(); //.map(|x| x.state() as u32);

    let mut img = RgbImage::new(W, H);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * H as usize) + x as usize;
        let color = colorous::MAGMA.eval_continuous(states[index] as f64 / STATES as f64);
        *pixel = Rgb([color.r, color.g, color.b]);
    });

    img.save(format!("hca_lifelike_test_{}.png", W)).unwrap();
}
