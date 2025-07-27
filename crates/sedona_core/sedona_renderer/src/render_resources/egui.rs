use wgpu::{Device, TextureFormat};

pub struct EguiRenderManager {
    // renderer: egui_wgpu::Renderer,
}

impl EguiRenderManager {
    pub fn new(surface_format: TextureFormat, device: &Device) -> Self {
        // let renderer = egui_wgpu::Renderer::new(device, surface_format, None, 1, true);

        Self {}
    }

    // pub fn renderer(&mut self) -> &mut egui_wgpu::Renderer {
    //     &mut self.renderer
    // }
}
