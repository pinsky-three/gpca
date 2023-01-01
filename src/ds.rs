use crate::dynamic::Dynamic;

use crate::space::DiscreteSpace;

#[derive(Debug, Clone)]
pub struct DynamicalSystemBuilder<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> {
    dimension: Box<T>,
    dynamic: Box<U>,
}

impl<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> DynamicalSystemBuilder<D, T, U> {
    pub fn new(space: T, dynamic: U) -> Self
    where
        T: DiscreteSpace<D>,
        U: Dynamic<D, T>,
    {
        Self {
            dimension: Box::new(space),
            dynamic: Box::new(dynamic),
        }
    }

    pub fn build(&self) -> DynamicalSystem<D, T, U>
    where
        T: Clone,
        U: Clone,
    {
        DynamicalSystem {
            // space: s,
            space: self.dimension.clone(),
            dynamic: self.dynamic.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystem<const D: usize, T: DiscreteSpace<D>, U: Dynamic<D, T>> {
    space: Box<T>,
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

    // pub fn space(&mut self) -> &[u32; D] {
    //     &self.space
    // }

    pub fn space(&mut self) -> &Box<T> {
        &self.space
    }

    pub fn tick(&mut self) {
        self.space
            .write_state(&self.dynamic.update(&self.space.read_state()))
    }
}

// pub struct DynamicalSystem {
//     dimension: Dimension,
//     n_m: Option<(i32, i32)>,
// }

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

// impl<const X: usize> Dynamic<1, dyn DiscreteSpace<1>>
//     for ElementaryCellularAutomaton<1, OneDimensional<X>>
// {
//     fn states(&self) -> u32 {
//         2
//     }

//     fn update(&self, input: &Vec<u32>) -> Vec<u32> {
//         let mut output: Vec<u32> = Vec::new();

//         for i in 0..input.len() {
//             let left = if i == 0 {
//                 input[input.len() - 1]
//             } else {
//                 input[i - 1]
//             };

//             let right = if i == input.len() - 1 {
//                 input[0]
//             } else {
//                 input[i + 1]
//             };

//             let index = (left << 2) | (input[i] << 1) | right;

//             // self.rule

//             output.push(self.rule[index as usize]);
//         }

//         output
//     }
// }

// impl<const X: usize> ElementaryCellularAutomaton<1, OneDimensional<X>> {
//     pub fn new(rule: [u32; 8]) -> Self {
//         Self {
//             rule,
//             phantom: PhantomData,
//         }
//     }

//     pub fn new_from_number(rule: u32) -> Self {
//         let mut rule_array = [0; 8];

//         for i in 0..8 {
//             rule_array[i] = (rule >> i) & 1;
//         }

//         Self::new(rule_array)
//     }
// }
