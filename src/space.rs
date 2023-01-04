use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

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

#[derive(Debug, Clone, Copy)]
pub struct OneDimensional<const X: usize> {
    space: [u32; X],
}

impl<const X: usize> OneDimensional<X> {
    pub fn new() -> Self {
        Self { space: [0; X] }
    }

    pub fn new_with_state(state: [u32; X]) -> Self {
        Self { space: state }
    }
}

#[derive(Debug, Clone)]
pub struct TwoDimensional<const X: usize, const Y: usize> {
    // space: Box<[[u32; X]; Y]>,
    space: Array2<u32>,
}

impl<const X: usize, const Y: usize> TwoDimensional<X, Y> {
    pub fn new() -> Self {
        Self {
            space: Array2::zeros((X, Y)),
        }
    }

    pub fn new_with_state(state: Array2<u32>) -> Self {
        Self {
            space: Array2::from(state),
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
        [self.space.len()]
    }

    fn read_state(&self) -> Vec<u32> {
        self.space.to_vec()
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.space = state.to_vec().as_slice().try_into().unwrap();
    }
}

impl<const X: usize, const Y: usize> DiscreteSpace<2> for TwoDimensional<X, Y> {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        self.space.shape().try_into().unwrap()
        // [self.space.len(), self.space.first().unwrap().len()]
    }

    fn read_state(&self) -> Vec<u32> {
        Vec::from(self.space.as_slice().unwrap())
        // self.space
        //     // .to_vec()
        //     .iter()
        //     .map(|r| r)
        //     .flatten()
        //     .collect::<Vec<u32>>()
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.space = Array2::from_shape_vec((X, Y), state.to_vec()).unwrap();
        // self.space = Box::new(
        //     state
        //         .to_vec()
        //         .chunks(X)
        //         .map(|r| r.try_into().unwrap())
        //         .collect::<Vec<[u32; X]>>()
        //         .as_slice()
        //         .try_into()
        //         .unwrap(),
        // );
    }
}

impl<const X: usize, const Y: usize> DiscreteSpaceArray<2> for TwoDimensional<X, Y> {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        self.space.shape().try_into().unwrap()
        // [self.space.len(), self.space.first().unwrap().len()]
    }

    fn read_state(&self) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> {
        self.space.clone()
    }

    fn write_state(&mut self, state: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>>) {
        self.space = state.clone();
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
