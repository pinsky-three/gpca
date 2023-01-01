use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum Dimension {
    One,
    Two,
    Three,
}

#[derive(Debug, Clone, Copy)]
pub struct OneDimensional(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct TwoDimensional(pub usize, pub usize);

#[derive(Debug, Clone, Copy)]
pub struct ThreeDimensional(pub usize, pub usize, pub usize);

pub trait DiscreteSpace<const D: usize> {
    fn dim(&self) -> Dimension;
    fn size(&self) -> [usize; D];
}

impl DiscreteSpace<1> for OneDimensional {
    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn size(&self) -> [usize; 1] {
        [self.0]
    }
}

impl DiscreteSpace<2> for TwoDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        [self.0, self.1]
    }
}

impl DiscreteSpace<3> for ThreeDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Three
    }

    fn size(&self) -> [usize; 3] {
        [self.0, self.1, self.2]
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystemBuilder<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> {
    dimension: Box<T>,
    dynamic: Box<U>,
    initial_state: Option<Vec<u32>>,
    size: [usize; D],
    // states: u32,
    // n_m: Vec<i32>,
    // n_m: Option<(i32, i32)>,
}

impl<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> DynamicalSystemBuilder<D, T, U> {
    pub fn new(space: T, dynamic: U) -> Self
    where
        T: DiscreteSpace<D>,
        U: Dynamic<D, T>,
    {
        let s = space.size();

        Self {
            dimension: Box::new(space),
            dynamic: Box::new(dynamic),
            initial_state: None,
            size: s,
        }
    }

    pub fn with_initial_state(&mut self, state: Vec<u32>) -> &mut Self {
        self.initial_state = Some(state);
        self
    }

    pub fn build(&self) -> DynamicalSystem<D, T, U>
    where
        T: Clone,
        U: Clone,
    {
        // let mut space = Vec::<Vec<u32>>::new();

        let s: Vec<u32> = match self.initial_state {
            Some(ref state) => state.clone(),
            None => self
                .dimension
                .size()
                .iter()
                .map(|d| vec![0; *d as usize])
                .flatten()
                .collect(),
        };

        DynamicalSystem {
            space: s,
            system: self.dimension.clone(),
            dynamic: self.dynamic.clone(),
            // dynamic: Box::new(self.dynamic.unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystem<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> {
    space: Vec<u32>,
    system: Box<T>,
    dynamic: Box<U>,
    // fn state() -> Vec<u64>;

    // fn update();
}

impl<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> DynamicalSystem<D, T, U> {
    // pub fn new<T>(builder: DynamicalSystemBuilder<T>) -> Self
    // where
    //     T: DiscreteSystem,
    // {
    //     Self {}
    // }

    // fn dims() -> dyn DiscreteSystem {}
    // fn dynamic() -> dyn Dynamic;

    pub fn space(&mut self) -> &Vec<u32> {
        &self.space
    }

    pub fn system(&mut self) -> &Box<T> {
        &self.system
    }

    pub fn tick(&mut self) {
        self.space = self.dynamic.update(&self.space);
    }
}

// pub struct DynamicalSystem {
//     dimension: Dimension,
//     n_m: Option<(i32, i32)>,
// }

pub trait Dynamic<const D: usize, S: DiscreteSpace<D>> {
    fn states(&self) -> u32;
    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomaton<const D: usize, S: DiscreteSpace<D>> {
    rule: [u32; 8],
    phantom: PhantomData<S>,
    // dimension: Box<T>,
}

// impl ElementaryCellularAutomaton<1, OneDimensional> {
//     pub fn new(rule: [u32; 8]) -> Self {
//         Self {
//             rule,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<const S: usize> ElementaryCellularAutomaton<S, OneDimensional> {
//     pub fn new(rule: [u32; 8]) -> Self {
//         Self {
//             rule,
//             phantom: PhantomData,
//         }
//     }
// }

impl<const D: usize> Dynamic<D, dyn DiscreteSpace<1>>
    for ElementaryCellularAutomaton<1, OneDimensional>
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, input: &Vec<u32>) -> Vec<u32> {
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

impl ElementaryCellularAutomaton<1, OneDimensional> {
    pub fn new(rule: [u32; 8]) -> Self {
        Self {
            rule,
            phantom: PhantomData,
        }
    }

    pub fn new_from_number(rule: u32) -> Self {
        let mut rule_array = [0; 8];

        for i in 0..8 {
            rule_array[i] = (rule >> i) & 1;
        }

        Self::new(rule_array)
    }
}
