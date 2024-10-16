use crate::spaces::{lattice::LatticeComputable, local::Stateable};

use std::hash::Hash;
use wgpu::util::DeviceExt;

pub type Real = f32;
pub enum BorderType {
    Crop,
    Mirror,
    Zero,
}

pub struct Image {
    pub data: Vec<Real>,
    pub width: u32,
    pub height: u32,
}

pub struct GpuDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

pub struct Kernel {
    pub data: Vec<Real>, // data length is size×size
    pub size: u32,
}

pub async fn run<N, E>(
    device: &GpuDevice,
    image: &Image,
    kernel: &Kernel,
    shader: wgpu::ShaderSource<'_>,

    dynamic: &impl LatticeComputable<N, E>,
) -> Image
where
    N: Clone + Sync + Send + Hash + Eq + Stateable,
    E: Clone + Sync + Send + Eq + PartialEq + Hash + Sized,
{
    let (mut output, output_size, result_buffer, output_buffer, bind_group, compute_pipeline) =
        dynamic.wgsl_compute(kernel, image, device, shader);
    // fun_name(kernel, image, device, shader);

    // encode and run commands
    let mut encoder = device
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.set_pipeline(&compute_pipeline);
        cpass.dispatch_workgroups(output.width, output.height, 1);
    }
    // copy data from input buffer on GPU to output buffer on CPU
    encoder.copy_buffer_to_buffer(&result_buffer, 0, &output_buffer, 0, output_size);
    device.queue.submit(Some(encoder.finish()));

    // read output_buffer
    let buffer_slice = output_buffer.slice(..);

    buffer_slice.map_async(wgpu::MapMode::Read, |_r| {});

    device.device.poll(wgpu::Maintain::Wait);

    let data = buffer_slice.get_mapped_range();
    output.data = bytemuck::cast_slice::<u8, f32>(&data).to_vec();

    // We have to make sure all mapped views are dropped before we unmap the buffer.
    drop(data);
    output_buffer.unmap();

    output
}

pub fn create_gpu_device() -> GpuDevice {
    let (device, queue) = futures::executor::block_on(create_device_queue());
    GpuDevice { device, queue }
}

pub async fn create_device_queue() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_limits: wgpu::Limits {
                    max_storage_buffer_binding_size: 512 * 1024 * 1024,
                    ..Default::default()
                },
                ..Default::default()
            },
            None,
        )
        .await
        .expect("Failed to create device")
}

impl GpuDevice {
    pub fn create_buffer(&self, label: &str, size: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        })
    }

    pub fn create_data_buffer(&self, label: &str, contents: &[u8]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            })
    }

    pub fn create_uniform_buffer(&self, label: &str, contents: &[u8]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents,
                usage: wgpu::BufferUsages::UNIFORM,
            })
    }

    pub fn create_output_buffer(&self, label: &str, size: u64) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn create_bind_group(
        &self,
        buffers: &[(&wgpu::Buffer, u64, wgpu::BufferBindingType)],
    ) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout_entries = buffers
            .iter()
            .enumerate()
            .map(|(index, (_, size, ty))| wgpu::BindGroupLayoutEntry {
                binding: index as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: *ty,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(*size),
                },
                count: None,
            })
            .collect::<Vec<_>>();
        let bind_group_layout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &layout_entries,
                });
        let group_entries = buffers
            .iter()
            .enumerate()
            .map(|(index, (buffer, _, _))| wgpu::BindGroupEntry {
                binding: index as u32,
                resource: buffer.as_entire_binding(),
            })
            .collect::<Vec<_>>();
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &group_entries,
        });
        (bind_group_layout, bind_group)
    }

    pub fn create_compute_pipeline(
        &self,
        buffers: &[(&wgpu::Buffer, u64, wgpu::BufferBindingType)],
        shader: wgpu::ShaderSource,
    ) -> (wgpu::BindGroup, wgpu::ComputePipeline) {
        let (bind_group_layout, bind_group) = self.create_bind_group(buffers);

        // create shader module
        let cs_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: shader, //wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader)),
                                // flags: wgpu::ShaderFlags::VALIDATION,
            });

        // create pipeline for shader
        let pipeline_layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let compute_pipeline =
            self.device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    module: &cs_module,
                    entry_point: "main",
                    cache: None,
                    compilation_options: wgpu::PipelineCompilationOptions {
                        ..Default::default()
                    },
                });

        (bind_group, compute_pipeline)
    }
}

impl std::ops::Index<(u32, u32)> for Image {
    type Output = Real;

    #[inline]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let idx = (y * self.width + x) as usize;
        &self.data[idx]
    }
}

impl std::ops::Index<(u32, u32)> for Kernel {
    type Output = Real;

    #[inline]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let idx = (y * self.size + x) as usize;
        &self.data[idx]
    }
}

/// Row major image data

// impl Image {
//     pub fn new(width: u32, height: u32, value: Real) -> Self {
//         let len = (width * height) as usize;
//         let data = vec![value; len];
//         Image {
//             width,
//             height,
//             data,
//         }
//     }

//     pub fn size(&self) -> u32 {
//         self.width * self.height
//     }

//     pub fn load<P: AsRef<std::path::Path>>(filepath: &P) -> Image {
//         let image = image::open(filepath).expect("read image file").into_luma8();
//         let (width, height) = image.dimensions();
//         let data = image.as_raw().iter().map(|pixel| *pixel as Real).collect();
//         Image {
//             data,
//             width,
//             height,
//         }
//     }

//     pub fn save<P: AsRef<std::path::Path>>(&self, filepath: P) {
//         let image = image::GrayImage::from_raw(
//             self.width,
//             self.height,
//             self.data.iter().map(|pixel| pixel.abs() as u8).collect(),
//         )
//         .expect("Create output image");
//         image.save(filepath).expect("write image file");
//     }
// }

// /// compute gaussian kernel
// pub fn gaussian(sigma: Real) -> Kernel {
//     /*
//       The size of the kernel is selected to guarantee that the first discarded
//       term is at least 10^prec times smaller than the central value. For that,
//       the half size of the kernel must be larger than x, with
//         e^(-x^2/2sigma^2) = 1/10^prec
//       Then,
//         x = sigma * sqrt( 2 * prec * ln(10) )
//     */
//     let prec = 3.0;
//     let radius = (sigma * (2.0 * prec * (10.0 as Real).ln()).sqrt()).ceil() as i32;
//     let size = 1 + 2 * radius; /* kernel size */
//     let mut data = Vec::with_capacity((size * size) as usize);
//     for y in -radius..=radius {
//         for x in -radius..=radius {
//             let dist2 = x.pow(2) + y.pow(2);
//             // proximate a circle region
//             let value = if dist2 <= radius * radius {
//                 (-0.5 * (dist2 as Real) / sigma.powi(2)).exp()
//             } else {
//                 0.0
//             };
//             data.push(value);
//         }
//     }

//     //normalization
//     let sum: Real = data.iter().sum();
//     if sum > 0.0 {
//         for v in data.iter_mut() {
//             *v /= sum;
//         }
//     }

//     Kernel {
//         data,
//         size: size as u32,
//     }
// }

// pub fn roberts_operator() -> (Kernel, Kernel) {
//     let kx = Kernel {
//         data: vec![1.0, 0.0, 0.0, -1.0],
//         size: 2,
//     };
//     let ky = Kernel {
//         data: vec![0.0, 1.0, -1.0, 0.0],
//         size: 2,
//     };
//     (kx, ky)
// }

// pub fn desolneux_operator() -> (Kernel, Kernel) {
//     let kx = Kernel {
//         data: vec![-0.5, 0.5, -0.5, 0.5],
//         size: 2,
//     };
//     let ky = Kernel {
//         data: vec![-0.5, -0.5, 0.5, 0.5],
//         size: 2,
//     };
//     (kx, ky)
// }

// pub fn sobel_operator() -> (Kernel, Kernel) {
//     let kx = Kernel {
//         data: vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
//         size: 3,
//     };
//     let ky = Kernel {
//         data: vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
//         size: 3,
//     };
//     (kx, ky)
// }

// pub fn freichen_operator() -> (Kernel, Kernel) {
//     let sqrt_2 = (2.0 as Real).sqrt();
//     let kx = Kernel {
//         data: vec![-1.0, 0.0, 1.0, -sqrt_2, 0.0, sqrt_2, -1.0, 0.0, 1.0],
//         size: 3,
//     };
//     let ky = Kernel {
//         data: vec![-1.0, -sqrt_2, -1.0, 0.0, 0.0, 0.0, 1.0, sqrt_2, 1.0],
//         size: 3,
//     };
//     (kx, ky)
// }

pub fn accumulation() -> Kernel {
    Kernel {
        data: vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        size: 3,
    }
}
