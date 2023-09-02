use anyhow::{anyhow, Result};

pub struct App {
    pub window: winit::window::Window,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl App {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Result<App> {
        use pollster::block_on;

        let window = winit::window::Window::new(event_loop)?;

        // Create a wgpu device that can render to that window.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env()
                .unwrap_or(wgpu::Backends::all()),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env()
                .unwrap_or_default(),
        });
        let surface = unsafe { instance.create_surface(&window) }?;
        let adapter = block_on(wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            Some(&surface),
        ))
        .ok_or(anyhow!("failed to create wgpu adapter"))?;
        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("play-wgpu device"),
                ..wgpu::DeviceDescriptor::default()
            },
            None,
        ))?;

        let app = App {
            window,
            surface,
            device,
            queue,
        };

        Ok(app)
    }
}
