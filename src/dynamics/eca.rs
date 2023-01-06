use std::marker::PhantomData;

use ndarray::{ArrayBase, Dim, OwnedRepr};

use crate::{
    dynamic::{Dynamic, DynamicArray},
    space::{DiscreteSpace, DiscreteSpaceArray, OneDimensional},
};

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomaton<S: DiscreteSpace<1>> {
    rule: [u32; 8],
    phantom: PhantomData<S>,
}

impl<const X: usize> ElementaryCellularAutomaton<OneDimensional<X>> {
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
    for ElementaryCellularAutomaton<OneDimensional<X>>
{
    fn name(&self) -> String {
        let r: u32 = self.rule.iter().map(|x| 1 << x).sum();

        format!("eca_{}", r)
    }

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

#[derive(Debug, Clone)]
pub struct ElementaryCellularAutomatonArray<S: DiscreteSpaceArray<1>> {
    rule: [u32; 8],
    phantom: PhantomData<S>,
}

impl<const X: usize> ElementaryCellularAutomatonArray<OneDimensional<X>> {
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

impl<const X: usize> DynamicArray<1, OneDimensional<X>>
    for ElementaryCellularAutomatonArray<OneDimensional<X>>
{
    fn name(&self) -> String {
        let r: u32 = self.rule.iter().enumerate().map(|(i, x)| x << i).sum();

        format!("eca_{}", r)
    }

    fn states(&self) -> u32 {
        2
    }

    fn update(
        &self,
        input: &ArrayBase<OwnedRepr<u32>, Dim<[usize; 1]>>,
    ) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; 1]>> {
        let mut output = ArrayBase::zeros(input.dim());

        for i in 0..input.len() {
            let left = if i == 0 {
                input[[input.len() - 1]]
            } else {
                input[[i - 1]]
            };

            let right = if i == input.len() - 1 {
                input[[0]]
            } else {
                input[[i + 1]]
            };

            let index = (left << 2) | (input[[i]] << 1) | right;

            output[[i]] = self.rule[index as usize];
        }

        output
    }
}
