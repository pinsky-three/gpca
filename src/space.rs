use std::ops::RangeBounds;

use ndarray::{Array1, Array2, ArrayBase, Dim, OwnedRepr};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    thread_rng, Rng,
};

pub trait DiscreteSpace<const D: usize> {
    fn dim(&self) -> Dimension;
    fn size(&self) -> [usize; D];

    fn read_state(&self) -> Vec<u32>;
    fn write_state(&mut self, state: &Vec<u32>);
    fn update_state(&mut self, updater: &mut dyn for<'a> FnMut(&'a mut Vec<u32>)) {
        let mut s = self.read_state();
        updater(&mut s);
        self.write_state(&s)
    }
}

pub trait DiscreteSpaceArray<const D: usize> {
    fn dim(&self) -> Dimension;
    fn size(&self) -> [usize; D];

    fn read_state(&self) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>;
    fn write_state(&mut self, state: &ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>);

    fn update_state(
        &mut self,
        updater: &mut dyn for<'a> FnMut(&'a mut ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>),
    ) {
        let mut s = self.read_state();
        updater(&mut s);
        self.write_state(&s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone)]
pub struct OneDimensional<const X: usize> {
    state: Array1<u32>,
}

impl<const X: usize> OneDimensional<X> {
    pub fn new() -> Self {
        Self {
            state: Array1::zeros(X),
        }
    }

    pub fn new_with_state(state: Array1<u32>) -> Self {
        Self { state }
    }
}

#[derive(Debug, Clone)]
pub struct TwoDimensional<const X: usize, const Y: usize> {
    state: Array2<u32>,
}

impl<const X: usize, const Y: usize> TwoDimensional<X, Y> {
    pub fn new() -> Self {
        Self {
            state: Array2::zeros((X, Y)),
        }
    }

    pub fn new_random(states: u32) -> Self {
        let mut rng = thread_rng();
        let mut state = Array2::zeros((X, Y));

        state.map_inplace(|x: &mut u32| *x = rng.gen_range(0..states) as u32);

        Self { state }
    }

    pub fn new_with_state(state: Array2<u32>) -> Self {
        Self {
            state: Array2::from(state),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThreeDimensional<const X: usize, const Y: usize, const Z: usize> {
    space: [[[u32; X]; Y]; Z],
}

impl<const X: usize> DiscreteSpace<1> for OneDimensional<X> {
    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn size(&self) -> [usize; 1] {
        [self.state.len()]
    }

    fn read_state(&self) -> Vec<u32> {
        self.state.to_vec()
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.state = Array1::from(state.to_vec());
    }
}

impl<const X: usize> DiscreteSpaceArray<1> for OneDimensional<X> {
    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn size(&self) -> [usize; 1] {
        self.state.shape().try_into().unwrap()
    }

    fn read_state(&self) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 1]>> {
        self.state.to_owned()
    }

    fn write_state(&mut self, state: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 1]>>) {
        self.state = state.to_owned();
    }
}

impl<const X: usize, const Y: usize> DiscreteSpace<2> for TwoDimensional<X, Y> {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        self.state.shape().try_into().unwrap()
    }

    fn read_state(&self) -> Vec<u32> {
        Vec::from(self.state.as_slice().unwrap())
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.state = Array2::from_shape_vec((X, Y), state.to_vec()).unwrap();
    }
}

impl<const X: usize, const Y: usize> DiscreteSpaceArray<2> for TwoDimensional<X, Y> {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        self.state.shape().try_into().unwrap()
        // [self.space.len(), self.space.first().unwrap().len()]
    }

    fn read_state(&self) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> {
        self.state.to_owned()
    }

    fn write_state(&mut self, state: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>) {
        self.state = state.to_owned();
    }
}

impl<const X: usize, const Y: usize, const Z: usize> DiscreteSpace<3>
    for ThreeDimensional<X, Y, Z>
{
    fn dim(&self) -> Dimension {
        Dimension::Three
    }

    fn read_state(&self) -> Vec<u32> {
        self.space
            .to_vec()
            .iter()
            .map(|r| {
                r.to_vec()
                    .iter()
                    .map(|c| c.to_vec())
                    .flatten()
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>()
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.space = state
            .to_vec()
            .chunks(X * Y)
            .map(|r| {
                r.to_vec()
                    .chunks(X)
                    .map(|c| c.try_into().unwrap())
                    .collect::<Vec<[u32; X]>>()
                    .as_slice()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[[u32; X]; Y]>>()
            .as_slice()
            .try_into()
            .unwrap();
    }

    fn size(&self) -> [usize; 3] {
        [
            self.space.len(),
            self.space.first().unwrap().len(),
            self.space.first().unwrap().first().unwrap().len(),
        ]
    }
}
