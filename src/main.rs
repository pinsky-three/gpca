use gpca::{
    haca::local::LocalHyperGraphHeap,
    haca_systems::life::{new_game_of_life_hyper_graph_heap, LifeState},
    third::wgpu::{self, accumulation, create_gpu_device, GpuDevice, Image},
};
use image::{Rgb, RgbImage};
use kdam::tqdm;
use rand::{rngs::ThreadRng, Rng};
use rayon::{
    iter::IntoParallelRefIterator,
    prelude::{IntoParallelRefMutIterator, ParallelIterator},
};

#[tokio::main]
async fn main() {
    const W: usize = 2048;
    const H: usize = W;

    const WH: usize = W * H;

    let mut mem = (0..WH).map(|_i| LifeState(0)).collect::<Vec<LifeState>>();

    mem.par_iter_mut().for_each(|x| {
        *x = if ThreadRng::default().gen_bool(0.5) {
            LifeState(1)
        } else {
            LifeState(0)
        }
    });

    let mut graph = new_game_of_life_hyper_graph_heap(mem);
    let device = create_gpu_device();

    for _ in tqdm!(0..1000) {
        graph.compute(&device, W as u32, H as u32);
    }

    let copy_mem = graph.nodes().par_iter().map(|x| x.0).collect::<Vec<u8>>();

    println!("copy mem len: {} | WxH: {}", copy_mem.len(), WH);

    let mut img = RgbImage::new(W as u32, H as u32);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * H) + x as usize;

        if copy_mem[index] == 1 {
            *pixel = Rgb([255, 255, 255]);
        } else {
            *pixel = Rgb([0, 0, 0]);
        }
    });

    img.save(format!("hca_game_of_life_test_{}.png", W))
        .unwrap();
}

trait LatticeComputable {
    fn compute(&mut self, device: &GpuDevice, w: u32, h: u32);
}

impl LatticeComputable for LocalHyperGraphHeap<LifeState, ()> {
    fn compute(&mut self, device: &GpuDevice, w: u32, h: u32) {
        let mem = self
            .nodes()
            .iter()
            .map(|x| x.0 as f32)
            .collect::<Vec<f32>>();

        let kernel = accumulation();

        let output = futures::executor::block_on(wgpu::run(
            device,
            &Image {
                data: mem,
                width: w,
                height: h,
            },
            &kernel,
        ));

        let res_data_len = output.data.len();
        let mut nodes = self.nodes().to_owned();

        nodes.iter_mut().enumerate().for_each(|(i, x)| {
            *x = LifeState(output.data[i % res_data_len] as u8);
        });

        self.update_nodes(nodes.clone());
    }
}
