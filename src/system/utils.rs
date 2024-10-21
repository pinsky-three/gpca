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

pub fn generate_image_from_space<N, D>(
    system: &System<N, D>,
    color_map: colorous::Gradient,
) -> RgbImage
where
    N: Stateable + Send + Sync + Clone + Debug,
    D: LocalDynamic<N, ()> + Debug + Clone,
{
    let (width, height) = system.space().payload();

    let current_full_state = system
        .space_state()
        .par_iter()
        .map(|x| x.state() as u8)
        .collect::<Vec<u8>>();

    let mut img = RgbImage::new(*width, *height);

    img.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let index = (y as usize * (*width as usize)) + x as usize;
        let color = color_map
            .eval_continuous(current_full_state[index] as f64 / system.dynamic().states() as f64);
        *pixel = Rgb([color.r, color.g, color.b]);
    });

    img
}
