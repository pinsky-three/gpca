use std::hash::Hash;

use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::{
    dynamics::local::LocalDynamic,
    spaces::{
        lattice::LatticeComputable,
        local::{HyperEdge, LocalHyperGraph, Stateable},
    },
    third::wgpu::{self, accumulation, GpuDevice, Image},
};

pub struct DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E>,
    D: LocalDynamic<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    _id: String,
    space: Box<S>,
    dynamic: Box<D>,
    phantom: std::marker::PhantomData<(N, E)>,
}

impl<S, D, N, E> DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E>,
    D: LocalDynamic<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
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
    S: LocalHyperGraph<N, E>,
    D: LocalDynamic<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    pub fn space_state(&self) -> Vec<N> {
        self.space.nodes().to_owned()
    }

    pub fn compute_sync(&mut self) {
        let mut new_nodes = self.space.nodes().clone();

        // let new_nodes = self
        //     .space
        //     .nodes()
        //     .par_iter()
        //     .enumerate()
        //     .map(|(id, node)| {
        //         self.dynamic.update(
        //             node,
        //             &self
        //                 .space
        //                 .node_neighbors()
        //                 .get(&id)
        //                 .unwrap()
        //                 .iter()
        //                 .map(|x| self.space.nodes()[*x].clone())
        //                 .collect::<Vec<N>>(),
        //             self.space
        //                 .edges()
        //                 .iter()
        //                 .filter(|(k, _)| *k == &id)
        //                 .map(|(_, v)| v)
        //                 .collect::<Vec<&HyperEdge<E>>>(),
        //         )
        //     })
        //     .collect();

        new_nodes.par_iter_mut().enumerate().for_each(|(i, node)| {
            let neighbors = self.space.node_neighbors().get(&i).unwrap().to_owned();
            let neighbor_nodes = neighbors
                .iter()
                .map(|i| self.space.nodes()[*i].clone())
                .collect::<Vec<N>>();

            // *node = node.interact(&neighbor_nodes, vec![]);
            *node = self.dynamic.update(node, &neighbor_nodes, vec![]);
        });

        self.space.update_nodes(new_nodes);
    }
}

impl<S, D, N, E> DynamicalSystem<S, D, N, E>
where
    S: LocalHyperGraph<N, E>,
    D: LocalDynamic<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: LatticeComputable<N, E>,
{
    pub fn compute_sync_wgpu(&mut self, device: &GpuDevice) {
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

        let output = futures::executor::block_on(wgpu::run(
            device,
            &Image {
                data: mem,
                width: w as u32,
                height: h as u32,
            },
            &kernel,
        ));

        let res_data_len = output.data.len();
        let mut nodes = self.space.nodes().to_owned();

        nodes.iter_mut().enumerate().for_each(|(i, x)| {
            *x = N::from_state(output.data[i % res_data_len] as u32); //LifeState( as u8);
        });

        self.space.update_nodes(nodes.clone());
    }
}
