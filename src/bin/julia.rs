use anyhow::{bail, Result};
use wgpu_hikari::app;
use winit::{dpi as Wd, event_loop as Wl};

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

struct Window {
    wgpu: wgpu_hikari::wgpu::Window,
    pipeline: wgpu::RenderPipeline,
}

impl Window {
    fn new(event_loop: &app::LoopTarget) -> Result<Window> {
        use pollster::block_on;

        // Create the winit window.
        let winit_window = winit::window::WindowBuilder::new()
            .with_inner_size(Wd::LogicalSize { width: 800, height: 450 })
            .build(event_loop)?;

        // Create the wgpu Device.
        let wgpu = wgpu_hikari::wgpu::Window::new(winit_window)?;

        wgpu.device.push_error_scope(wgpu::ErrorFilter::Validation);

        // Create the shader module.
        let mut wgsl_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wgsl_path.push("src/bin/julia.wgsl");
        let wgsl = std::fs::read_to_string(std::path::Path::new(&wgsl_path))?;
        let shader_module = wgpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Julia shader module"),
            source: wgpu::ShaderSource::Wgsl(wgsl.into()),
        });

        // Create the render pipeline.
        let pipeline = wgpu.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Julia render pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "julia_vertex",
                buffers: &[], // We generate vertex positions from the instance index.
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                .. wgpu::PrimitiveState::default()
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
                })]
            }),
            multiview: None,
        });
        
        if let Some(err) = block_on(wgpu.device.pop_error_scope()) {
            bail!("{}", err);
        }

        Ok(Window {
            wgpu,
            pipeline,
        })
    }
}


#[allow(unused_variables)]
impl wgpu_hikari::window::Window for Window {
    fn id(&self) -> winit::window::WindowId {
        self.wgpu.winit_window.id()
    }

    fn resized(
        &mut self,
        _event_loop: &app::LoopTarget,
        size: Wd::PhysicalSize<u32>,
    ) -> Result<Option<Wl::ControlFlow>> {
        log::trace!("resized");
        Ok(None)
    }

    fn redraw(&mut self, event_loop: &app::LoopTarget) -> Result<Option<Wl::ControlFlow>> {
        use pollster::block_on;

        log::trace!("redraw");
        self.wgpu.device.push_error_scope(wgpu::ErrorFilter::Validation);

        let frame = self.wgpu.surface.get_current_texture()?;
        let frame_view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Julia frame texture view"),
            .. wgpu::TextureViewDescriptor::default()
        });

        let mut command_encoder = self.wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                .. wgpu::RenderPassDescriptor::default()
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..4, 0..1);
        }

        self.wgpu.queue.submit(Some(command_encoder.finish()));
        frame.present();

        if let Some(err) = block_on(self.wgpu.device.pop_error_scope()) {
            bail!("{}", err);
        }

        Ok(None)
    }
}
