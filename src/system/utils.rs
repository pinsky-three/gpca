use std::fmt::Debug;

use image::{Rgb, RgbImage};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    dynamics::local::LocalDynamic,
    spaces::{implementations::basic::HyperGraphHeap, local::Stateable},
};

use super::dynamical_system::DynamicalSystem;

type Space<N> = HyperGraphHeap<N, (), (u32, u32)>;
type System<N, D> = DynamicalSystem<Space<N>, D, N, ()>;

pub fn save_space_as_image<N, D>(system: &System<N, D>)
where
    N: Stateable + Send + Sync + Clone + Debug,
    D: LocalDynamic<N, ()> + Debug,
{
    let (width, height) = system.space().payload();

    let current_full_state = system
        .space_state()
        .par_iter()
        .map(|x| x.state() as u8)
        .collect::<Vec<u8>>();

    let mut img = RgbImage::new(*width, *height);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * 2048) + x as usize;
        let color =
            colorous::RED_YELLOW_BLUE.eval_continuous(current_full_state[index] as f64 / 4.0);
        *pixel = Rgb([color.r, color.g, color.b]);
    });

    img.save("hca_lifelike_test.png").unwrap();
}
