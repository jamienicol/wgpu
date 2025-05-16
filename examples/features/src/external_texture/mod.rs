use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vertex {
    _pos: [f32; 2],
    _tex_coord: [f32; 2],
}

fn create_vertices() -> Vec<Vertex> {
    vec![
        Vertex {
            _pos: [-0.5, -0.5],
            _tex_coord: [0.0, 1.0],
        },
        Vertex {
            _pos: [-0.5, 0.5],
            _tex_coord: [0.0, 0.0],
        },
        Vertex {
            _pos: [0.5, 0.5],
            _tex_coord: [1.0, 0.0],
        },
        Vertex {
            _pos: [0.5, -0.5],
            _tex_coord: [1.0, 1.0],
        },
    ]
}

fn create_indices() -> Vec<u16> {
    vec![0, 1, 2, 2, 0, 3]
}

struct Example {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_format: wgpu::IndexFormat,
}

impl crate::framework::Example for Example {
    fn optional_features() -> wgpu::Features {
        wgpu::Features::empty()
    }
    fn required_features() -> wgpu::Features {
        wgpu::Features::EXTERNAL_TEXTURE
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::downlevel_defaults()
    }
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let shader_module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let vertex_size = size_of::<Vertex>();
        let vertex_data = create_vertices();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_data = create_indices();
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let texture_descriptor = wgpu::TextureDescriptor {
            size: wgpu::Extent3d::default(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
            view_formats: &[],
        };
        let red_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("red"),
            view_formats: &[],
            ..texture_descriptor
        });
        let red_texture_view = red_texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            red_texture.as_image_copy(),
            &[255, 0, 0, 255],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: None,
            },
            wgpu::Extent3d::default(),
        );

        let external_texture_desc = wgpu::ExternalTextureDescriptor {
            label: Some("External texture"),
            width: 512,
            height: 512,
            format: wgpu::ExternalTextureFormat::Rgba,
            yuv_conversion_matrix: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
            gamut_conversion_matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            src_transfer_function: wgpu::ExternalTextureTransferFunction::default(),
            dst_transfer_function: wgpu::ExternalTextureTransferFunction::default(),
            sample_transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            load_transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        };

        let external_texture =
            device.create_external_texture(&external_texture_desc, &[&red_texture_view]);

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::ExternalTexture,
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::ExternalTexture(&external_texture),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            layout: &bind_group_layout,
            label: Some("bind group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("main"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let index_format = wgpu::IndexFormat::Uint16;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: Some("vert_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: vertex_size as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: Some("frag_main"),
                compilation_options: Default::default(),
                targets: &[Some(config.view_formats[0].into())],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            vertex_buffer,
            index_buffer,
            index_format,
        }
    }
    fn resize(
        &mut self,
        _sc_desc: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        // noop
    }
    fn update(&mut self, _event: winit::event::WindowEvent) {
        // noop
    }
    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("primary"),
        });

        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        rpass.set_pipeline(&self.pipeline);
        rpass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        rpass.set_index_buffer(self.index_buffer.slice(..), self.index_format);

        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw_indexed(0..6, 0, 0..1);

        drop(rpass);

        queue.submit(Some(encoder.finish()));
    }
}

pub fn main() {
    crate::framework::run::<Example>("texture-arrays");
}

#[cfg(test)]
fn test_parameters() -> wgpu_test::TestParameters {
    wgpu_test::TestParameters::default()
        // https://github.com/gfx-rs/wgpu/issues/7287
        .expect_fail(wgpu_test::FailureCase::backend_adapter(
            wgpu::Backends::METAL,
            "Apple M",
        ))
}

#[cfg(test)]
#[wgpu_test::gpu_test]
static TEST: crate::framework::ExampleTestParams = crate::framework::ExampleTestParams {
    name: "texture-arrays",
    image_path: "/examples/features/src/texture_arrays/screenshot.png",
    width: 1024,
    height: 768,
    optional_features: wgpu::Features::empty(),
    base_test_parameters: test_parameters(),
    comparisons: &[wgpu_test::ComparisonType::Mean(0.0)],
    _phantom: std::marker::PhantomData::<Example>,
};

#[cfg(test)]
#[wgpu_test::gpu_test]
static TEST_UNIFORM: crate::framework::ExampleTestParams = crate::framework::ExampleTestParams {
    name: "texture-arrays-uniform",
    image_path: "/examples/features/src/texture_arrays/screenshot.png",
    width: 1024,
    height: 768,
    optional_features: wgpu::Features::empty(),
    base_test_parameters: test_parameters(),
    comparisons: &[wgpu_test::ComparisonType::Mean(0.0)],
    _phantom: std::marker::PhantomData::<Example>,
};

#[cfg(test)]
#[wgpu_test::gpu_test]
static TEST_NON_UNIFORM: crate::framework::ExampleTestParams =
    crate::framework::ExampleTestParams {
        name: "texture-arrays-non-uniform",
        image_path: "/examples/features/src/texture_arrays/screenshot.png",
        width: 1024,
        height: 768,
        optional_features:
            wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
        base_test_parameters: test_parameters(),
        comparisons: &[wgpu_test::ComparisonType::Mean(0.0)],
        _phantom: std::marker::PhantomData::<Example>,
    };
