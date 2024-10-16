use std::{borrow::Cow, hash::Hash};

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

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct LifeLike {
    b_list: &'static [u32],
    s_list: &'static [u32],
}

impl LifeLike {
    pub fn new(b_list: &'static [u32], s_list: &'static [u32]) -> Self {
        Self { b_list, s_list }
    }
}

impl<N, E> LocalDynamic<N, E> for LifeLike
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
    Self: Clone + Send + Sync + Hash + Eq + PartialEq,
{
    fn states(&self) -> u32 {
        2
    }

    fn update(&self, node: &N, nodes: &[N], _edges: Vec<&HyperEdge<E>>) -> N {
        let total = nodes.iter().map(|n| n.state()).sum();

        if self.b_list.contains(&total) {
            let a: u32 = 1; //self.states() - 1;
            N::from_state(a)
        } else if self.s_list.contains(&total) {
            node.clone()
        } else {
            N::from_state(0)
        }
    }
}

impl<N, E> LatticeComputable<N, E>
    for DynamicalSystem<HyperGraphHeap<DiscreteState, (), (u32, u32)>, LifeLike, DiscreteState, ()>
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
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
        wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("lifelike.wgsl")))
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

        // let n: Vec<i32> = self
        //     .observation_neighbors()
        //     .into_iter()
        //     .flatten()
        //     .collect::<Vec<i32>>();

        // let n: Vec<Vec<i32>> = self.observation_neighbors();

        // let a = n.iter().flatten().copied().collect::<Vec<i32>>();

        let a = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];

        // let b_list = [3u32];
        // let s_list = [2u32, 3u32];

        let b_num = self
            .dynamic()
            .b_list
            .iter()
            .fold(0, |acc, &b| acc | (1 << b));

        let s_num = self
            .dynamic()
            .s_list
            .iter()
            .fold(0, |acc, &s| acc | (1 << s));

        let output_size = (output.size() * std::mem::size_of::<Real>() as u32) as u64;
        let params = vec![image.width, image.height, 8, b_num, s_num];
        let params_data = bytemuck::cast_slice(&params);

        // create input and output buffers
        let input_buffer = device.create_data_buffer("input", bytemuck::cast_slice(&image.data));
        let result_buffer = device.create_buffer("result", output_size);
        // let neighbors_buffer = device.create_data_buffer("neighbors", bytemuck::cast_slice(&n));
        let neighbors_buffer = device.create_data_buffer("neighbors", bytemuck::cast_slice(&a[..]));
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
