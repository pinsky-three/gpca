use crate::space::DiscreteSpace;

pub trait Dynamic<const D: usize, S: DiscreteSpace<D>> {
    fn states(&self) -> u32;
    fn update(&self, input: &Vec<u32>) -> Vec<u32>;
}
