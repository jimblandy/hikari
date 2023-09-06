use anyhow::{bail, Result};
use encase::ShaderType;
use wgpu_hikari::app;
use winit::{dpi as Wd, event as We, event_loop as Wl};

struct App;

fn main() {
    env_logger::init();
    app::run(App);
}

impl app::App for App {
    type Window = Window;

    fn create_first_window(&mut self, event_loop: &app::LoopTarget) -> Result<Self::Window> {
        log::trace!("create_first_window");
        Window::new(event_loop)
    }
}

#[derive(ShaderType)]
struct Uniform {
    /// Transformation from clip space to the complex plane.
    transform: glam::Mat3,

    /// Position of the mouse on the complex plane.
    mouse: glam::Vec3,
}

impl Uniform {
    /// Compute the uniform contents given mouse position and window size.
    ///
    /// Given:
    ///
    /// - `mouse`, the mouse position in pixels relative to the
    ///   upper-left corner of the window, and
    ///
    /// - `window_size`, the size of the window in pixels,
    ///
    /// Return a `Uniform` value initialized according to its
    /// documentation.
    fn new(mouse: Wd::PhysicalPosition<f64>,
           window_size: Wd::PhysicalSize<u32>) -> Self {
        let size = glam::Vec2 { x: window_size.width as f32, y: window_size.height as f32 };
        let mouse = glam::Vec2 { x: mouse.x as f32, y: mouse.y as f32 };

        // We want the plane unit square (i.e., of diameter 2) centered on the
        // origin to always sit in the middle of the window with its aspect
        // ratio preserved. We assume pixels are square.
        let clip_to_plane_scale = if size.x >= size.y {
            glam::Vec2 { x: size.x / size.y, y: 1.0 }
        } else {
            glam::Vec2 { x: 1.0, y: size.y / size.x }
        };
        let clip_to_plane_scale = clip_to_plane_scale * 2.0;

        let mouse_clip = (mouse / size - 0.5) * glam::Vec2 { x: 2.0, y: -2.0 };
        let mouse_plane = mouse_clip * clip_to_plane_scale;

        Uniform {
            transform: glam::Mat3::from_scale(clip_to_plane_scale),
            mouse: mouse_plane.extend(1.0),
        }
    }
}

struct Window {
    wgpu: wgpu_hikari::wgpu::Window,
    pipeline: wgpu::RenderPipeline,
    buffer: wgpu::Buffer,
    bindgroup: wgpu::BindGroup,

    mouse: Wd::PhysicalPosition<f64>,
    window_size: Wd::PhysicalSize<u32>,
}

impl Window {
    fn new(event_loop: &app::LoopTarget) -> Result<Window> {
        use pollster::block_on;

        // Create the winit window.
        let winit_window = winit::window::WindowBuilder::new()
            .with_title("Julia set")
            .with_inner_size(Wd::LogicalSize {
                width: 1200,
                height: 675,
            })
            .with_position(Wd::LogicalPosition { x: 50, y: 50 })
            .build(event_loop)?;

        // Create the wgpu Device.
        let wgpu = wgpu_hikari::wgpu::Window::new(winit_window)?;

        wgpu.device.push_error_scope(wgpu::ErrorFilter::Validation);

        // Create the shader module.
        let mut wgsl_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wgsl_path.push("src/bin/julia.wgsl");
        let wgsl = std::fs::read_to_string(std::path::Path::new(&wgsl_path))?;
        let shader_module = wgpu
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Julia shader module"),
                source: wgpu::ShaderSource::Wgsl(wgsl.into()),
            });

        // Create the render pipeline.
        let pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Julia render pipeline"),
                layout: None, // Use the automatic bindgroup layout.
                vertex: wgpu::VertexState {
                    module: &shader_module,
                    entry_point: "julia_vertex",
                    buffers: &[], // We generate vertex positions from the instance index.
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip,
                    ..wgpu::PrimitiveState::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: "julia_fragment",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu.surface_configuration.format,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
            });

        // Create the uniform buffer.
        let buffer = wgpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Julia uniform buffer"),
            size: Uniform::min_size().get(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        // Create the bindgroup.
        let layout = wgpu
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Julia bindgroup layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(Uniform::min_size()),
                    },
                    count: None,
                }],
            });
        let bindgroup = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Julia bindgroup"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        if let Some(err) = block_on(wgpu.device.pop_error_scope()) {
            bail!("{}", err);
        }

        let window_size = Wd::PhysicalSize {
            width: wgpu.surface_configuration.width,
            height: wgpu.surface_configuration.height,
        };
        
        Ok(Window {
            wgpu,
            buffer,
            bindgroup,
            pipeline,
            mouse: Wd::PhysicalPosition::default(),
            window_size,
        })
    }
}

#[allow(unused_variables)]
impl wgpu_hikari::window::Window for Window {
    fn id(&self) -> winit::window::WindowId {
        self.wgpu.winit_window.id()
    }

    fn cursor_moved(
        &mut self,
        event_loop: &app::LoopTarget,
        device_id: We::DeviceId,
        position: Wd::PhysicalPosition<f64>,
    ) -> Result<Option<Wl::ControlFlow>> {
        self.mouse = position;
        self.wgpu.winit_window.request_redraw();
        Ok(None)
    }

    fn redraw(&mut self, event_loop: &app::LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        use pollster::block_on;

        log::trace!("redraw");
        self.wgpu
            .device
            .push_error_scope(wgpu::ErrorFilter::Validation);

        let frame = self.wgpu.surface.get_current_texture()?;
        let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Julia frame texture view"),
            ..wgpu::TextureViewDescriptor::default()
        });

        // Compute our new uniform contents.
        let uniform = Uniform::new(self.mouse, self.window_size);
        log::trace!("    mouse: {:?}", self.mouse);
        log::trace!("    window size: {:?}", self.window_size);
        log::trace!("    mouse on plane: {}", uniform.mouse);

        // Write our current mouse position and transform to the uniform buffer.
        {
            let mut bytes = Vec::with_capacity(uniform.size().get() as usize);
            let mut buffer = encase::UniformBuffer::new(&mut bytes);
            buffer.write(&uniform).unwrap();

            self.wgpu.queue.write_buffer(&self.buffer, 0, &bytes);
        }

        let mut command_encoder =
            self.wgpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Julia redraw"),
                });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Julia render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: true,
                    },
                })],
                ..wgpu::RenderPassDescriptor::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bindgroup, &[]);
            render_pass.draw(0..4, 0..1);
        }

        self.wgpu.queue.submit(Some(command_encoder.finish()));
        frame.present();

        if let Some(err) = block_on(self.wgpu.device.pop_error_scope()) {
            bail!("{}", err);
        }

        Ok(None)
    }

    fn resized(
        &mut self,
        _event_loop: &app::LoopTarget,
        size: Wd::PhysicalSize<u32>,
    ) -> Result<Option<Wl::ControlFlow>> {
        log::trace!("resized: {:?}", size);
        self.window_size = size;
        self.wgpu.resize(size);
        Ok(None)
    }
}
