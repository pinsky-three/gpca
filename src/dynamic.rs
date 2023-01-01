use std::marker::PhantomData;

use crate::space::{DiscreteSpace, OneDimensional};

pub trait Dynamic<const D: usize, S: DiscreteSpace<D>> {
    fn states(&self) -> u32;
    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomaton<const D: usize, S: DiscreteSpace<D>> {
    rule: [u32; 8],
    phantom: PhantomData<S>,
}

impl<const X: usize> ElementaryCellularAutomaton<1, OneDimensional<X>> {
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

impl<const X: usize> Dynamic<1, OneDimensional<X>>
    for ElementaryCellularAutomaton<1, OneDimensional<X>>
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
