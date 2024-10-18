use std::{borrow::Cow, fmt::Debug, hash::Hash};

use wgpu::ShaderSource;

use crate::{
    dynamics::local::LocalDynamic,
    spaces::{
        implementations::basic::{DiscreteState, HyperGraphHeap},
        lattice::LatticeComputable,
        local::{HyperEdge, Stateable},
    },
    system::dynamical_system::DynamicalSystem,
    third::wgpu::{GpuDevice, Image, Kernel, Real},
};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct CyclicAutomaton {
    states: u32,
    threshold: u32,
}

impl CyclicAutomaton {
    pub fn new(states: u32, threshold: u32) -> Self {
        Self { states, threshold }
    }
}

impl<N, E> LocalDynamic<N, E> for CyclicAutomaton
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Clone + Send + Sync + Hash + Eq + PartialEq,
{
    fn states(&self) -> u32 {
        self.states
    }

    fn update(&self, node: &N, nodes: &[N], _edges: Vec<&HyperEdge<E>>) -> N {
        let total_successors = nodes
            .iter()
            .map(|n| n.state())
            .filter(|&n| n == (node.state() + 1) % self.states)
            .count();

        if total_successors >= self.threshold as usize {
            let a: u32 = (node.state() + 1) % self.states;
            N::from_state(a)
        } else {
            node.clone()
        }
    }
}

impl<N, E> LatticeComputable<N, E>
    for DynamicalSystem<
        HyperGraphHeap<DiscreteState, (), (u32, u32)>,
        CyclicAutomaton,
        DiscreteState,
        (),
    >
where
    N: Clone + Sync + Send + Hash + Eq + Stateable + Debug,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized + Debug,
{
    fn shape(&self) -> Vec<usize> {
        let (w, h) = self.space().payload();

        vec![*w as usize, *h as usize]
    }

    fn observation_neighbors(&self) -> Vec<Vec<i32>> {
        vec![
            vec![-1, -1],
            vec![-1, 0],
            vec![-1, 1],
            vec![0, -1],
            vec![0, 1],
            vec![1, -1],
            vec![1, 0],
            vec![1, 1],
        ]
    }

    fn update_wgsl_code(&self) -> ShaderSource {
        wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("cyclic.wgsl")))
    }

    fn wgsl_compute(
        &self,
        kernel: &Kernel,
        image: &Image,
        device: &GpuDevice,
        shader: wgpu::ShaderSource,
    ) -> (
        Image,
        u64,
        wgpu::Buffer,
        wgpu::Buffer,
        wgpu::BindGroup,
        wgpu::ComputePipeline,
    ) {
        let crop = kernel.size - 1;

        let output = Image {
            data: Vec::new(),
            width: image.width - crop,
            height: image.height - crop,
        };

        let observation = <_ as LatticeComputable<N, E>>::observation_neighbors(self);

        let neighbors = observation.iter().flatten().copied().collect::<Vec<i32>>();

        let b_num = self.dynamic().states;
        let s_num = self.dynamic().threshold;

        let output_size =
            (output.width * output.height * std::mem::size_of::<Real>() as u32) as u64;
        let params = vec![image.width, image.height, 8, b_num, s_num];
        let params_data = bytemuck::cast_slice(&params);

        // create input and output buffers
        let input_buffer = device.create_data_buffer("input", bytemuck::cast_slice(&image.data));
        let result_buffer = device.create_buffer("result", output_size);
        // let neighbors_buffer = device.create_data_buffer("neighbors", bytemuck::cast_slice(&n));
        let neighbors_buffer =
            device.create_data_buffer("neighbors", bytemuck::cast_slice(&neighbors[..]));
        let output_buffer = device.create_output_buffer("output", output_size);

        // let rule_data = vec![b_num, s_num];

        // let rule_buffer = device.create_data_buffer("rule", bytemuck::cast_slice(&rule_data));
        let params_buffer = device.create_uniform_buffer("params", params_data);

        // create bind group and compute pipeline
        let (bind_group, compute_pipeline) = device.create_compute_pipeline(
            &[
                (
                    &input_buffer,
                    4,
                    wgpu::BufferBindingType::Storage { read_only: false },
                ),
                (
                    &result_buffer,
                    4,
                    wgpu::BufferBindingType::Storage { read_only: false },
                ),
                (
                    &neighbors_buffer,
                    8,
                    wgpu::BufferBindingType::Storage { read_only: false },
                ),
                (
                    &params_buffer,
                    params_data.len() as u64,
                    wgpu::BufferBindingType::Uniform,
                ),
                // (
                //     &rule_buffer,
                //     rule_data.len() as u64,
                //     wgpu::BufferBindingType::Storage { read_only: true },
                // ),
            ],
            shader,
        );
        (
            output,
            output_size,
            result_buffer,
            output_buffer,
            bind_group,
            compute_pipeline,
        )
    }
}
