pub type Real = f32;
pub enum BorderType {
    Crop,
    Mirror,
    Zero,
}

pub async fn run(device: &GpuDevice, image: &Image, kernel: &Kernel) -> Image {
    let crop = kernel.size - 1;

    let mut output = Image {
        data: Vec::new(),
        width: image.width - crop,
        height: image.height - crop,
    };

    let neighbors_example: Vec<[i32; 2]> = vec![
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let output_size = (output.size() * std::mem::size_of::<Real>() as u32) as u64;
    let params = [image.width, image.height, 8];
    let params_data = bytemuck::cast_slice(&params);

    // create input and output buffers
    let input_buffer = device.create_data_buffer("input", bytemuck::cast_slice(&image.data));
    let result_buffer = device.create_buffer("result", output_size);
    let neighbors_buffer =
        device.create_data_buffer("neighbors", bytemuck::cast_slice(&neighbors_example));
    let params_buffer = device.create_uniform_buffer("params", params_data);
    let output_buffer = device.create_output_buffer("output", output_size);

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
        ],
        include_str!("convolution.wgsl"),
    );

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

use std::borrow::Cow;
use wgpu::util::DeviceExt;

pub struct GpuDevice {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
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
        shader: &str,
    ) -> (wgpu::BindGroup, wgpu::ComputePipeline) {
        let (bind_group_layout, bind_group) = self.create_bind_group(buffers);

        // create shader module
        let cs_module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader)),
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

/// Row major image data
pub struct Image {
    pub data: Vec<Real>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32, value: Real) -> Self {
        let len = (width * height) as usize;
        let data = vec![value; len];
        Image {
            width,
            height,
            data,
        }
    }

    pub fn size(&self) -> u32 {
        self.width * self.height
    }

    pub fn load<P: AsRef<std::path::Path>>(filepath: &P) -> Image {
        let image = image::open(filepath).expect("read image file").into_luma8();
        let (width, height) = image.dimensions();
        let data = image.as_raw().iter().map(|pixel| *pixel as Real).collect();
        Image {
            data,
            width,
            height,
        }
    }

    pub fn save<P: AsRef<std::path::Path>>(&self, filepath: P) {
        let image = image::GrayImage::from_raw(
            self.width,
            self.height,
            self.data.iter().map(|pixel| pixel.abs() as u8).collect(),
        )
        .expect("Create output image");
        image.save(filepath).expect("write image file");
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

/// Square shaped convolution kernel
pub struct Kernel {
    pub data: Vec<Real>, // data length is size×size
    pub size: u32,
}

impl std::ops::Index<(u32, u32)> for Kernel {
    type Output = Real;

    #[inline]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let idx = (y * self.size + x) as usize;
        &self.data[idx]
    }
}

/// compute gaussian kernel
pub fn gaussian(sigma: Real) -> Kernel {
    /*
      The size of the kernel is selected to guarantee that the first discarded
      term is at least 10^prec times smaller than the central value. For that,
      the half size of the kernel must be larger than x, with
        e^(-x^2/2sigma^2) = 1/10^prec
      Then,
        x = sigma * sqrt( 2 * prec * ln(10) )
    */
    let prec = 3.0;
    let radius = (sigma * (2.0 * prec * (10.0 as Real).ln()).sqrt()).ceil() as i32;
    let size = 1 + 2 * radius; /* kernel size */
    let mut data = Vec::with_capacity((size * size) as usize);
    for y in -radius..=radius {
        for x in -radius..=radius {
            let dist2 = x.pow(2) + y.pow(2);
            // proximate a circle region
            let value = if dist2 <= radius * radius {
                (-0.5 * (dist2 as Real) / sigma.powi(2)).exp()
            } else {
                0.0
            };
            data.push(value);
        }
    }

    //normalization
    let sum: Real = data.iter().sum();
    if sum > 0.0 {
        for v in data.iter_mut() {
            *v /= sum;
        }
    }

    Kernel {
        data,
        size: size as u32,
    }
}

pub fn roberts_operator() -> (Kernel, Kernel) {
    let kx = Kernel {
        data: vec![1.0, 0.0, 0.0, -1.0],
        size: 2,
    };
    let ky = Kernel {
        data: vec![0.0, 1.0, -1.0, 0.0],
        size: 2,
    };
    (kx, ky)
}

pub fn desolneux_operator() -> (Kernel, Kernel) {
    let kx = Kernel {
        data: vec![-0.5, 0.5, -0.5, 0.5],
        size: 2,
    };
    let ky = Kernel {
        data: vec![-0.5, -0.5, 0.5, 0.5],
        size: 2,
    };
    (kx, ky)
}

pub fn sobel_operator() -> (Kernel, Kernel) {
    let kx = Kernel {
        data: vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0],
        size: 3,
    };
    let ky = Kernel {
        data: vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0],
        size: 3,
    };
    (kx, ky)
}

pub fn freichen_operator() -> (Kernel, Kernel) {
    let sqrt_2 = (2.0 as Real).sqrt();
    let kx = Kernel {
        data: vec![-1.0, 0.0, 1.0, -sqrt_2, 0.0, sqrt_2, -1.0, 0.0, 1.0],
        size: 3,
    };
    let ky = Kernel {
        data: vec![-1.0, -sqrt_2, -1.0, 0.0, 0.0, 0.0, 1.0, sqrt_2, 1.0],
        size: 3,
    };
    (kx, ky)
}

pub fn accumulation() -> Kernel {
    Kernel {
        data: vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        size: 3,
    }
}

pub struct Pipeline {
    pub device: GpuDevice,
    encoder: wgpu::CommandEncoder,
}

impl Pipeline {
    pub fn new() -> Self {
        let device = create_gpu_device();
        let encoder = device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        Pipeline { device, encoder }
    }

    pub fn chain(
        &mut self,
        input_buffer: &wgpu::Buffer,
        kernel: &Kernel,
        image_size: (u32, u32),
    ) -> (wgpu::Buffer, (u32, u32)) {
        let (width, height) = image_size;
        let crop = kernel.size - 1;
        let output = Image {
            data: Vec::new(),
            width: width - crop,
            height: height - crop,
        };
        let output_size = (output.size() * std::mem::size_of::<Real>() as u32) as u64;
        let result_buffer = self.device.create_buffer("result", output_size);
        let kernel_buffer = self
            .device
            .create_data_buffer("kernel", bytemuck::cast_slice(&kernel.data));
        let params = [width, kernel.size];
        let params_data = bytemuck::cast_slice(&params);
        let params_buffer = self.device.create_uniform_buffer("params", params_data);

        // create bind group and compute pipeline
        let (bind_group, compute_pipeline) = self.device.create_compute_pipeline(
            &[
                (
                    input_buffer,
                    4,
                    wgpu::BufferBindingType::Storage { read_only: true },
                ),
                (
                    &result_buffer,
                    4,
                    wgpu::BufferBindingType::Storage { read_only: false },
                ),
                (
                    &kernel_buffer,
                    4,
                    wgpu::BufferBindingType::Storage { read_only: true },
                ),
                (
                    &params_buffer,
                    params_data.len() as u64,
                    wgpu::BufferBindingType::Uniform,
                ),
            ],
            include_str!("convolution.wgsl"),
        );

        let mut cpass = self
            .encoder
            .begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                ..Default::default()
            });
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.set_pipeline(&compute_pipeline);
        cpass.dispatch_workgroups(output.width, output.height, 1);

        (result_buffer, (output.width, output.height))
    }

    pub async fn run<T: bytemuck::Pod>(
        mut self,
        output_buffers: &[(&wgpu::Buffer, (u32, u32), u32)],
    ) -> Vec<Vec<T>> {
        let mut output_offset_sizes = Vec::with_capacity(output_buffers.len());
        let mut offset = 0;
        for (result, image_size, pixel_size) in output_buffers {
            let size = (image_size.0 * image_size.1 * pixel_size) as u64;
            output_offset_sizes.push((result, offset, size));
            offset += size;
        }
        let output_buffer = self.device.create_output_buffer("output", offset);
        for (result, offset, size) in output_offset_sizes {
            self.encoder
                .copy_buffer_to_buffer(result, 0, &output_buffer, offset, size);
        }
        self.device.queue.submit(Some(self.encoder.finish()));

        // Read output
        let buffer_slice = output_buffer.slice(..);

        buffer_slice.map_async(wgpu::MapMode::Read, |_r| {});

        self.device.device.poll(wgpu::Maintain::Wait);

        // Awaits until `buffer_future` can be read from
        let data = buffer_slice.get_mapped_range();
        let mut output = bytemuck::cast_slice::<u8, T>(&data).to_vec();

        // We have to make sure all mapped views are dropped before we unmap the buffer.
        drop(data);
        output_buffer.unmap();

        let mut outputs = Vec::with_capacity(output_buffers.len());
        for (_, image_size, _) in output_buffers {
            let size = (image_size.0 * image_size.1) as usize;
            let remained_data = output.split_off(size);
            outputs.push(output);
            output = remained_data;
        }
        outputs
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
