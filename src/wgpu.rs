use std::sync::Arc;

use anyhow::{anyhow, Result};
use winit::{dpi as Wd, window as Ww};

// This doesn't really make sense, because you'd like to share your
// wgpu device/adapter across many surfaces. And you'd like the app to
// define its own window state type, with shared application state.
pub struct Window {
    pub winit_window: Arc<Ww::Window>,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub surface_capabilities: wgpu::SurfaceCapabilities,
    pub surface_configuration: wgpu::SurfaceConfiguration,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Window {
    /// Create a new `wgpu::Window` built around `winit_window`.
    pub fn new(winit_window: Ww::Window) -> Result<Window> {
        use pollster::block_on;

        let winit_window = Arc::new(winit_window);

        // Create a wgpu device that can render to that window.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all()),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
            ..wgpu::InstanceDescriptor::default()
        });
        let surface = instance.create_surface(winit_window.clone())?;
        let adapter = block_on(wgpu::util::initialize_adapter_from_env_or_default(
            &instance,
            Some(&surface),
        ))
        .ok_or(anyhow!("failed to create wgpu adapter"))?;
        log::debug!("Window::new adapter: {:#?}", adapter.get_info());
        let (device, queue) = block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("play-wgpu device"),
                ..wgpu::DeviceDescriptor::default()
            },
            None,
        ))?;

        let surface_capabilities = surface.get_capabilities(&adapter);
        let size = winit_window.inner_size();
        let surface_configuration = surface.get_default_config(&adapter, size.width, size.height).unwrap();
        surface.configure(&device, &surface_configuration);

        Ok(Window {
            winit_window,
            instance,
            surface,
            surface_capabilities,
            surface_configuration,
            adapter,
            device,
            queue,
        })
    }

    pub fn resize(&mut self, size: Wd::PhysicalSize<u32>) {
        self.surface_configuration.width = size.width;
        self.surface_configuration.height = size.height;
        self.surface
            .configure(&self.device, &self.surface_configuration);
        self.winit_window.request_redraw();
    }
}
