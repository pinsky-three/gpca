use std::{fmt::Debug, hash::Hash, ops::Deref};

use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::{
    dynamics::local::LocalDynamic,
    spaces::{
        lattice::LatticeComputable,
        local::{LocalHyperGraph, Stateable},
    },
    third::wgpu::{self, accumulation, GpuDevice, Image},
};

#[derive(Debug, Clone)]
pub struct DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E> + Debug + Clone,
    D: LocalDynamic<N, E> + Debug + Clone,
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    _id: String,
    space: Box<S>,
    dynamic: Box<D>,
    phantom: std::marker::PhantomData<(N, E)>,
}

impl<S, D, N, E> DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E> + Debug + Clone,
    D: LocalDynamic<N, E> + Debug + Clone,
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    pub fn new(space: Box<S>, dynamic: Box<D>) -> Self {
        Self {
            _id: "DynamicalSystem".to_string(),
            space,
            dynamic,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<S, D, N, E> DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E> + Debug + Clone,
    D: LocalDynamic<N, E> + Debug + Clone,
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    pub fn space_state(&self) -> Vec<N> {
        self.space.nodes().to_owned()
    }

    pub fn space(&self) -> &S {
        &self.space
    }

    pub fn dynamic(&self) -> &D {
        &self.dynamic
    }

    pub fn describe(&self) -> String {
        format!("{:?}", self)
    }

    pub fn set_dynamic(&mut self, dynamic: Box<D>) {
        self.dynamic = dynamic
    }

    pub fn set_space(&mut self, space: Box<S>) {
        self.space = space
    }

    pub fn update_space(&mut self, mutator: impl FnMut(&mut Vec<N>)) {
        self.space.update_nodes_mut(mutator);
    }

    pub fn compute_sync(&mut self) {
        let mut new_nodes = self.space.nodes().clone();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let neighbors = self.space.node_neighbors().get(&i).unwrap().to_owned();

            let neighbor_nodes = neighbors
                .iter()
                .map(|i| self.space.nodes()[*i].clone())
                .collect::<Vec<N>>();

            *node = self.dynamic.update(node, &neighbor_nodes, vec![]);
        });

        self.space.update_nodes(new_nodes);
    }
}

impl<S, D, N, E> DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E> + Debug + Clone,
    D: LocalDynamic<N, E> + Debug + Clone,
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
    Self: LatticeComputable<N, E>,
{
    pub async fn compute_sync_wgpu(&mut self, device: &GpuDevice) {
        let shape = self.shape();

        if shape.len() != 2 {
            panic!("Only 2D lattices are supported for now");
        }

        let (w, h) = (shape[0], shape[1]);

        let mem = self
            .space
            .nodes()
            .iter()
            .map(|x| x.state() as f32)
            .collect::<Vec<f32>>();

        let kernel = accumulation();

        let output = wgpu::run(
            device,
            &Image {
                data: mem,
                width: w as u32,
                height: h as u32,
            },
            &kernel,
            self.update_wgsl_code(),
            self.deref(),
        )
        .await;

        let res_data_len = output.data.len();
        let mut nodes = self.space.nodes().to_owned();

        nodes.iter_mut().enumerate().for_each(|(i, x)| {
            *x = N::from_state(output.data[i % res_data_len] as u32); //LifeState( as u8);
        });

        self.space.update_nodes(nodes.clone());
    }
}
