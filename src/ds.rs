use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy)]
pub struct OneDimensional(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct TwoDimensional(pub u32, pub u32);

#[derive(Debug, Clone, Copy)]
pub struct ThreeDimensional(pub u32, pub u32, pub u32);

pub trait DiscreteSpace {
    fn dim(&self) -> Dimension;
    fn size(&self) -> Vec<u32>;
}

impl DiscreteSpace for OneDimensional {
    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn size(&self) -> Vec<u32> {
        vec![self.0]
    }
}

impl DiscreteSpace for TwoDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> Vec<u32> {
        vec![self.0, self.1]
    }
}

impl DiscreteSpace for ThreeDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Three
    }

    fn size(&self) -> Vec<u32> {
        vec![self.0, self.1, self.2]
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystemBuilder<T: DiscreteSpace, U: Dynamic<T>> {
    dimension: Box<T>,
    dynamic: Box<U>,
    // states: u32,
    // n_m: Vec<i32>,
    // n_m: Option<(i32, i32)>,
}

impl<T: DiscreteSpace, U: Dynamic<T>> DynamicalSystemBuilder<T, U> {
    pub fn new(space: T, dynamic: U) -> Self
    where
        T: DiscreteSpace,
        U: Dynamic<T>,
    {
        Self {
            dimension: Box::new(space),
            dynamic: Box::new(dynamic),
            // states: 2,
            // n_m: vec![1, 1],
            // n_m: None,
        }
    }

    pub fn build(&self) -> DynamicalSystem<T, U>
    where
        T: Clone,
        U: Clone,
    {
        // let mut space = Vec::<Vec<u32>>::new();

        let s: Vec<u32> = self
            .dimension
            .size()
            .iter()
            .map(|d| vec![0; *d as usize])
            .flatten()
            .collect();

        DynamicalSystem {
            space: s,
            system: self.dimension.clone(),
            dynamic: self.dynamic.clone(),
            // dynamic: Box::new(self.dynamic.unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystem<T: DiscreteSpace, U: Dynamic<T>> {
    space: Vec<u32>,
    system: Box<T>,
    dynamic: Box<U>,
    // fn state() -> Vec<u64>;

    // fn update();
}

impl<T: DiscreteSpace, U: Dynamic<T>> DynamicalSystem<T, U> {
    // pub fn new<T>(builder: DynamicalSystemBuilder<T>) -> Self
    // where
    //     T: DiscreteSystem,
    // {
    //     Self {}
    // }

    // fn dims() -> dyn DiscreteSystem {}
    // fn dynamic() -> dyn Dynamic;

    // fn space() -> Vec<u64>;

    // fn tick();
}

// pub struct DynamicalSystem {
//     dimension: Dimension,
//     n_m: Option<(i32, i32)>,
// }

pub trait Dynamic<T: DiscreteSpace> {
    fn states(&self) -> u32;

    fn update(&self, input: Vec<u32>) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomaton<T: DiscreteSpace> {
    rule: [u32; 8],
    phantom: PhantomData<T>,
    // dimension: Box<T>,
}

impl Dynamic<OneDimensional> for ElementaryCellularAutomaton<OneDimensional> {
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, input: Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();

        for i in 0..input.len() {
            let left = if i == 0 {
                input[input.len() - 1]
            } else {
                input[i - 1]
            };

            let right = if i == input.len() - 1 {
                input[0]
            } else {
                input[i + 1]
            };

            let index = (left << 2) | (input[i] << 1) | right;

            // self.rule

            output.push(self.rule[index as usize]);
        }

        output
    }
}

impl ElementaryCellularAutomaton<OneDimensional> {
    pub fn new(rule: [u32; 8]) -> Self {
        Self {
            rule,
            phantom: PhantomData,
        }
    }
}
