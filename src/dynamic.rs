use std::marker::PhantomData;

use crate::space::{DiscreteSpace, OneDimensional};

pub trait Dynamic<const D: usize, S: DiscreteSpace<D>> {
    fn states(&self) -> u32;
    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}
