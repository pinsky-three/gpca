use gpca::{
    dynamics::implementations::cyclic::CyclicAutomaton,
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
    const W: u32 = 2048;
    const H: u32 = 2048;

    const STATES: u32 = 4;
    const THRESH: u32 = 3;

    let _device = create_gpu_device();

    let space = HyperGraphHeap::new_grid(&DiscreteState::filled_vector(W * H, STATES), W, H, ());

    // let dynamic = LifeLike::new(&[3], &[2, 3, 7, 8]); // highlife+

    let dynamic = CyclicAutomaton::new(STATES, THRESH);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    // println!("system: {:?}", system.describe());

    for _ in tqdm!(0..500) {
        system.compute_sync_wgpu(&_device);
        // system.compute_sync();
    }

    let current_full_state = system
        .space_state()
        .par_iter()
        .map(|x| x.state() as u8)
        .collect::<Vec<u8>>();

    let mut img = RgbImage::new(W, H);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * W as usize) + x as usize;
        let color = colorous::RED_YELLOW_BLUE
            .eval_continuous(current_full_state[index] as f64 / STATES as f64);
        *pixel = Rgb([color.r, color.g, color.b]);
    });

    img.save(format!("hca_lifelike_test_{}.png", W)).unwrap();
}

// fn save_space_as_image<N>(state_space: &Vec<N>) {
//     let current_full_state = state_space
//         .par_iter()
//         .map(|x| x.state() as u8)
//         .collect::<Vec<u8>>();

//     let mut img = RgbImage::new(space.width(), space.height());

//     img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
//         let index = (y as usize * space.width() as usize) + x as usize;
//         let color = colorous::COOL
//             .eval_continuous(current_full_state[index] as f64 / space.states() as f64);
//         *pixel = Rgb([color.r, color.g, color.b]);
//     });

//     img.save(format!("hca_lifelike_test_{}.png", space.width()))
//         .unwrap();
// }
