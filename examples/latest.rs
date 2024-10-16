use gpca::{
    dynamics::implementations::cyclic::CyclicAutomaton,
    spaces::implementations::basic::{DiscreteState, HyperGraphHeap},
    system::{dynamical_system::DynamicalSystem, utils::save_space_as_image},
    third::wgpu::create_gpu_device,
};

use kdam::tqdm;

#[tokio::main]
async fn main() {
    const W: u32 = 2048;
    const H: u32 = 1024;

    const STATES: u32 = 4;
    const THRESH: u32 = 2;

    let _device = create_gpu_device();

    let mem = DiscreteState::filled_vector(W * H, STATES);
    let space = HyperGraphHeap::new_grid(&mem, W, H, ());

    let dynamic = CyclicAutomaton::new(STATES, THRESH);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    for _ in tqdm!(0..500) {
        system.compute_sync_wgpu(&_device);
        // system.compute_sync();
    }

    save_space_as_image(&system, colorous::PLASMA);
}
