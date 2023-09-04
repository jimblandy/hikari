use anyhow::{anyhow, Result};
use winit::window as Ww;

pub struct Window {
    pub winit_window: Ww::Window,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface,
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

        // Create a wgpu device that can render to that window.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all()),
            dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });
        let surface = unsafe { instance.create_surface(&winit_window) }?;
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
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::default(),
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
        };
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
}
