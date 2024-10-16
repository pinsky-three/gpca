use std::hash::Hash;

use wgpu::ShaderSource;

use crate::third::wgpu::{GpuDevice, Image, Kernel};

use super::local::Stateable;

// use super::local::{Interaction, LocalHyperGraphHeapTrait};

pub trait LatticeComputable<N, E>
where
    // M: LocalHyperGraphHeapTrait<N, E>,
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    fn shape(&self) -> Vec<usize>;
    fn observation_neighbors(&self) -> Vec<Vec<i32>>;
    fn update_wgsl_code(&self) -> ShaderSource;
    fn wgsl_compute<'a>(
        &self,
        kernel: &'a Kernel,
        image: &'a Image,
        device: &'a GpuDevice,
        shader: wgpu::ShaderSource,
    ) -> (
        Image,
        u64,
        wgpu::Buffer,
        wgpu::Buffer,
        wgpu::BindGroup,
        wgpu::ComputePipeline,
    );
}
