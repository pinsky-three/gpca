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

pub trait DiscreteSpace<const S: usize> {
    fn dim(&self) -> Dimension;
    fn size(&self) -> [usize; S];
}

impl DiscreteSpace<1> for OneDimensional {
    fn dim(&self) -> Dimension {
        Dimension::One
    }

    fn size(&self) -> [usize; 1] {
        vec![self.0]
    }
}

impl DiscreteSpace<2> for TwoDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Two
    }

    fn size(&self) -> [usize; 2] {
        vec![self.0, self.1]
    }
}

impl DiscreteSpace<3> for ThreeDimensional {
    fn dim(&self) -> Dimension {
        Dimension::Three
    }

    fn size(&self) -> [usize; 3] {
        vec![self.0, self.1, self.2]
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystemBuilder<const S: usize, T: DiscreteSpace<S>, U: Dynamic<S, T>> {
    dimension: Box<T>,
    dynamic: Box<U>,
    initial_state: Option<Vec<u32>>,
    size: Vec<u32>,
    // states: u32,
    // n_m: Vec<i32>,
    // n_m: Option<(i32, i32)>,
}

impl<const S: usize, T: DiscreteSpace<S>, U: Dynamic<S, T>> DynamicalSystemBuilder<S, T, U> {
    pub fn new(space: T, dynamic: U) -> Self
    where
        T: DiscreteSpace<S>,
        U: Dynamic<S, T>,
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

    pub fn build(&self) -> DynamicalSystem<S, T, U>
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
pub struct DynamicalSystem<const S: usize, T: DiscreteSpace<S>, U: Dynamic<S, T>> {
    space: Vec<u32>,
    system: Box<T>,
    dynamic: Box<U>,
    // fn state() -> Vec<u64>;

    // fn update();
}

impl<const S: usize, T: DiscreteSpace<S>, U: Dynamic<S, T>> DynamicalSystem<S, T, U> {
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

pub trait Dynamic<const S: usize, T: DiscreteSpace<S>> {
    fn states(&self) -> u32;

    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomaton<const S: usize, T: DiscreteSpace<S>> {
    rule: [u32; 8],
    phantom: PhantomData<T>,
    // dimension: Box<T>,
}

impl<const S: usize, OneDimensional> Dynamic<S, dyn DiscreteSpace<S>>
    for ElementaryCellularAutomaton<S, OneDimensional>
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

impl<const S: usize> ElementaryCellularAutomaton<S, OneDimensional> {
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
