use ndarray::{ArrayBase, Dim, OwnedRepr};

use crate::space::{DiscreteSpace, DiscreteSpaceArray};

pub trait Dynamic<const D: usize, S: DiscreteSpace<D>> {
    fn name(&self) -> String;
    fn states(&self) -> u32;
    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}

pub trait DynamicArray<const D: usize, S: DiscreteSpaceArray<D>> {
    fn name(&self) -> String;
    fn states(&self) -> u32;
    fn update(
        &self,
        input: &ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>,
    ) -> ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>;
}
