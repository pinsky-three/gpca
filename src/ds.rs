use std::time::Instant;

use ndarray::{ArrayBase, Dim, OwnedRepr};

use crate::dynamic::{Dynamic, DynamicArray};

use crate::space::{DiscreteSpace, DiscreteSpaceArray};

#[derive(Debug, Clone)]
pub struct DynamicalSystemBuilder<const D: usize, S: DiscreteSpace<D>, F: Dynamic<D, S>> {
    space: Box<S>,
    dynamic: Box<F>,
}

impl<const D: usize, S: DiscreteSpace<D>, F: Dynamic<D, S>> DynamicalSystemBuilder<D, S, F> {
    pub fn new(space: S, dynamic: F) -> Self
    where
        S: DiscreteSpace<D>,
        F: Dynamic<D, S>,
    {
        Self {
            space: Box::new(space),
            dynamic: Box::new(dynamic),
        }
    }

    pub fn update_state(&mut self, updater: &mut dyn for<'a> FnMut(&mut Vec<u32>)) -> &Self {
        self.space.update_state(updater);
        self
    }

    pub fn build(&self) -> DynamicalSystem<D, S, F>
    where
        S: Clone,
        F: Clone,
    {
        DynamicalSystem {
            // space: s,
            space: self.space.clone(),
            dynamic: self.dynamic.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystem<const D: usize, S: DiscreteSpace<D>, F: Dynamic<D, S>> {
    space: Box<S>,
    dynamic: Box<F>,
}

impl<const D: usize, S: DiscreteSpace<D>, F: Dynamic<D, S>> DynamicalSystem<D, S, F> {
    pub fn space(&mut self) -> &Box<S> {
        &self.space
    }

    pub fn tick(&mut self) {
        let state = self.space.read_state();

        let next_state = self.dynamic.update(&state);

        self.space.write_state(&next_state)
    }

    pub fn name(&self) -> String {
        format!(
            "{}_{}",
            self.dynamic.name(),
            self.space
                .size()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("_"),
        )
    }

    pub fn evolve(&mut self, steps: usize) -> f64 {
        let now = Instant::now();

        for _ in 0..steps {
            self.tick();
        }

        let elapsed = now.elapsed();

        steps as f64 / elapsed.as_secs_f64()
    }
}

#[derive(Debug, Clone)]
pub struct DynamicalSystemArray<const D: usize, S: DiscreteSpaceArray<D>, F: DynamicArray<D, S>> {
    space: Box<S>,
    dynamic: Box<F>,
}

impl<const D: usize, S: DiscreteSpaceArray<D>, F: DynamicArray<D, S>>
    DynamicalSystemArray<D, S, F>
{
    pub fn space(&mut self) -> &Box<S> {
        &self.space
    }

    pub fn tick(&mut self) {
        let state = self.space.read_state();

        let next_state = self.dynamic.update(&state);

        self.space.write_state(&next_state)
    }

    pub fn name(&self) -> String {
        format!(
            "{}_{}",
            self.dynamic.name(),
            self.space
                .size()
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("_"),
        )
    }

    pub fn evolve(&mut self, steps: usize) -> f64 {
        let now = Instant::now();

        for _ in 0..steps {
            self.tick();
        }

        let elapsed = now.elapsed();

        steps as f64 / elapsed.as_secs_f64()
    }
}

pub struct DynamicalSystemArrayBuilder<
    const D: usize,
    S: DiscreteSpaceArray<D>,
    F: DynamicArray<D, S>,
> {
    space: Box<S>,
    dynamic: Box<F>,
}

impl<const D: usize, S: DiscreteSpaceArray<D>, F: DynamicArray<D, S>>
    DynamicalSystemArrayBuilder<D, S, F>
{
    pub fn new(space: S, dynamic: F) -> Self
    where
        S: DiscreteSpaceArray<D>,
        F: DynamicArray<D, S>,
    {
        Self {
            space: Box::new(space),
            dynamic: Box::new(dynamic),
        }
    }

    pub fn update_state(
        &mut self,
        updater: &mut dyn for<'a> FnMut(&mut ArrayBase<OwnedRepr<u32>, Dim<[usize; D]>>),
    ) -> &Self {
        self.space.update_state(updater);
        self
    }

    pub fn build(&self) -> DynamicalSystemArray<D, S, F>
    where
        S: Clone,
        F: Clone,
    {
        DynamicalSystemArray {
            // space: s,
            space: self.space.clone(),
            dynamic: self.dynamic.clone(),
        }
    }
}
