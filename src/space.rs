pub trait DiscreteSpace<const D: usize> {
    fn dim(&self) -> Dimension;
    fn size(&self) -> [usize; D];

    fn read_state(&self) -> Vec<u32>;
    fn write_state(&mut self, state: &Vec<u32>);
    fn update_state(&mut self, updater: &dyn Fn(&mut Vec<u32>)) {
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

    // pub fn new_with_init(mutator: &dyn Fn(&mut [u32; X])) -> Self {
    //     let mut space = [0; X];

    //     mutator(&mut space);

    //     Self { space }
    // }

    pub fn new_with_state(state: [u32; X]) -> Self {
        Self { space: state }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TwoDimensional<const X: usize, const Y: usize> {
    space: [[u32; X]; Y],
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
        [self.space.len(), self.space.first().unwrap().len()]
    }

    fn read_state(&self) -> Vec<u32> {
        self.space
            .to_vec()
            .iter()
            .map(|r| r.to_vec())
            .flatten()
            .collect::<Vec<u32>>()
    }

    fn write_state(&mut self, state: &Vec<u32>) {
        self.space = state
            .to_vec()
            .chunks(X)
            .map(|r| r.try_into().unwrap())
            .collect::<Vec<[u32; X]>>()
            .as_slice()
            .try_into()
            .unwrap();
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
