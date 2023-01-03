use std::marker::PhantomData;

use crate::{
    dynamic::Dynamic,
    space::{DiscreteSpace, TwoDimensional},
};

pub struct LifeLikeCellularAutomaton<S: DiscreteSpace<2>> {
    b_list: [u32; 8],
    s_list: [u32; 8],
    phantom: PhantomData<S>,
}

impl<const X: usize, const Y: usize> LifeLikeCellularAutomaton<TwoDimensional<X, Y>> {
    pub fn new(b_list: [u32; 8], s_list: [u32; 8]) -> Self {
        Self {
            b_list,
            s_list,
            phantom: PhantomData,
        }
    }
}

impl<const X: usize, const Y: usize> Dynamic<2, TwoDimensional<X, Y>>
    for LifeLikeCellularAutomaton<TwoDimensional<X, Y>>
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, input: &Vec<u32>) -> Vec<u32> {
        let mut output: Vec<u32> = Vec::new();

        for i in 0..input.len() {
            let mut neighbors = 0;

            let x = i % X;
            let y = i / X;

            for j in 0..X {
                for k in 0..Y {
                    if j == x && k == y {
                        continue;
                    }

                    let index = (j * X) + k;

                    if input[index] == 1 {
                        neighbors += 1;
                    }
                }
            }

            if input[i] == 1 {
                if self.s_list[neighbors as usize] == 1 {
                    output.push(1);
                } else {
                    output.push(0);
                }
            } else {
                if self.b_list[neighbors as usize] == 1 {
                    output.push(1);
                } else {
                    output.push(0);
                }
            }
        }

        output
    }
}
