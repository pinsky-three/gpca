use gpca::{
    dynamics::implementations::cyclic::CyclicAutomaton,
    spaces::{
        implementations::basic::{DiscreteState, HyperGraphHeap},
        local::Stateable,
    },
    system::{dynamical_system::DynamicalSystem, utils::generate_image_from_space},
    third::wgpu::create_gpu_device,
};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[tokio::main]
async fn main() {
    const W: u32 = 1024;
    const H: u32 = 1024;

    const STATES: u32 = 4;
    const THRESH: u32 = 2;

    let _device = create_gpu_device().await;

    let mem = (0..W * H)
        .into_par_iter()
        .map(|_| DiscreteState::from_state(rand::thread_rng().gen_range(0..STATES)))
        .collect();

    let space = HyperGraphHeap::new_grid(&mem, W, H, ());

    let dynamic = CyclicAutomaton::new(STATES, THRESH);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    for _ in 0..500 {
        system.compute_sync_wgpu(&_device).await;
        // system.compute_sync();
    }

    let img = generate_image_from_space(&system, &|val| {
        let val = ((val.state() * 255) / STATES) as u8;
        image::Rgb([val, val, val])
    });

    img.save("output.png").unwrap();
}
