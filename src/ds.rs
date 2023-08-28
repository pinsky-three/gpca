use crate::dynamic::Dynamic;

use crate::space::DiscreteSpace;

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
    pub fn space(&mut self) -> &S {
        &self.space
    }

    pub fn tick(&mut self) {
        let state = self.space.read_state();

        let next_state = self.dynamic.update(&state);

        self.space.write_state(&next_state)
    }
}
