use anyhow::Result;

pub struct Display {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swapchain: wgpu::SwapChain,
}

impl Display {
    pub async fn new(window: &winit::window::Window) -> Result<Display> {
        let size = window.inner_size();

        let inst = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { inst.create_surface(window) };
        let adapter = inst.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            }
        ).await.expect("Unable to find adapter.");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        ).await?;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        let swapchain = device.create_swap_chain(&surface, &sc_desc);

        let d = Display {
            surface,
            device,
            queue,
            sc_desc,
            swapchain
        };

        Ok(d)
    }

    pub fn reload_swapchain(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swapchain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}