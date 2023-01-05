use crate::space::{DiscreteSpace, DiscreteSpaceArray, TwoDimensional};
use image::{ImageBuffer, Rgb, RgbImage};

pub trait ImageSpaceRenderer<const D: usize> {
    fn render(&self, space: &dyn DiscreteSpace<D>) -> RgbImage;
}
pub trait ImageSpaceArrayRenderer<const D: usize> {
    fn render(&self) -> RgbImage;
}

impl<const X: usize, const Y: usize> ImageSpaceArrayRenderer<2> for TwoDimensional<X, Y> {
    fn render(&self) -> RgbImage {
        let state = DiscreteSpaceArray::read_state(self);
        let size = DiscreteSpaceArray::size(self);

        ImageBuffer::from_fn(size[0] as u32, size[1] as u32, |x, y| {
            if state[[x as usize, y as usize]] == 1 {
                Rgb([255, 255, 255])
            } else {
                Rgb([0, 0, 0])
            }
        })
    }
}
