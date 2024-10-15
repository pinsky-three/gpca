use gpca::{
    dynamics::implementations::life::LifeLike,
    spaces::{
        implementations::basic::{DiscreteState, HyperGraphHeap},
        local::Stateable,
    },
    system::dynamical_system::DynamicalSystem,
    third::wgpu::create_gpu_device,
    // third::wgpu::create_gpu_device,
};
use image::{Rgb, RgbImage};
use kdam::tqdm;
use rand::{rngs::ThreadRng, Rng};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

// use image::{buffer::ConvertBuffer, ImageBuffer};
// use image::{Rgb, RgbImage};
// use kdam::tqdm;
// use rand::{rngs::ThreadRng, Rng};
// use rayon::{
//     iter::IntoParallelRefIterator,
//     prelude::{IntoParallelRefMutIterator, ParallelIterator},
// };

#[tokio::main]
async fn main() {
    const W: u32 = 2048;
    const H: u32 = W;

    const WH: u32 = W * H;

    const STATES: usize = 2;

    let mem = (0..WH)
        .into_par_iter()
        .map(|_i| DiscreteState::from_state(ThreadRng::default().gen_range(0..STATES) as u32))
        .collect::<Vec<DiscreteState>>();

    let space = HyperGraphHeap::new_grid(&mem, W, H, ());

    println!("Space created");

    let device = create_gpu_device();

    // mem.par_iter_mut().for_each(|x| {
    //     let val = ;

    //     *x = LifeState(val as u8);
    // });

    // let mut graph = new_game_of_life_hyper_graph_heap(mem);

    let dynamic = LifeLike::new(&[3], &[2, 3]);

    let mut system = DynamicalSystem::new(Box::new(space), Box::new(dynamic));

    println!("System created");

    for _ in tqdm!(0..100) {
        // graph.compute(&device, W as u32, H as u32);
        // graph.compute_with_neighbors().await;
        system.compute_sync_wgpu(&device);

        // system.compute_sync();
    }

    let states = system
        .space_state()
        .par_iter()
        .map(|x| x.state() as u8)
        .collect::<Vec<u8>>(); //.map(|x| x.state() as u32);

    // let copy_mem: Vec<u8> = graph.nodes().par_iter().map(|x| x.0).collect::<Vec<u8>>();

    let mut img = RgbImage::new(W, H);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * H as usize) + x as usize;
        *pixel = color_map(states[index], STATES as u8);
    });

    // let img: ImageBuffer<Rgb<u8>, Vec<u8>> = img.convert();

    img.save(format!("hca_lifelike_test_{}.png", W)).unwrap();
}

fn color_map(val: u8, states: u8) -> Rgb<u8> {
    let gradient = colorous::MAGMA;
    let color = gradient.eval_continuous(val as f64 / states as f64);

    Rgb([color.r, color.g, color.b])
}

// trait LatticeComputable {
//     fn compute(&mut self, device: &GpuDevice, w: u32, h: u32);
// }

// impl LatticeComputable for LocalHyperGraphHeap<LifeState, ()> {
//     fn compute(&mut self, device: &GpuDevice, w: u32, h: u32) {
//         let mem = self
//             .nodes()
//             .iter()
//             .map(|x| x.0 as f32)
//             .collect::<Vec<f32>>();

//         let kernel = accumulation();

//         let output = futures::executor::block_on(wgpu::run(
//             device,
//             &Image {
//                 data: mem,
//                 width: w,
//                 height: h,
//             },
//             &kernel,
//         ));

//         let res_data_len = output.data.len();
//         let mut nodes = self.nodes().to_owned();

//         nodes.iter_mut().enumerate().for_each(|(i, x)| {
//             *x = LifeState(output.data[i % res_data_len] as u8);
//         });

//         self.update_nodes(nodes.clone());
//     }
// }
