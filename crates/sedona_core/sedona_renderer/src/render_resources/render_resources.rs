use crate::render_resources::*;
use sedona_settings::Settings;
use wgpu::{Device, Queue, TextureFormat};

pub struct RenderResources {
    pub settings: RenderSettings,
    pub layouts: BindGroupLayouts,
    pub shaders: Shaders,
    pub targets: RenderTargets,
    pub globals: RenderGlobals,
    pub objects: RenderObjects,
    pub materials: RenderMaterials,
    pub blit: BlitRenderManager,
    pub pre_pass: PrePassRenderManager,
    pub shadow: ShadowRenderManager,
    pub sky: SkyRenderManager,
    pub post_process: PostProcessRenderManager,
    pub egui: EguiRenderManager,
}

impl RenderResources {
    pub fn new(
        window_width: u32,
        window_height: u32,
        config: &Settings,
        surface_format: TextureFormat,
        device: &Device,
        queue: &Queue,
    ) -> Self {
        let settings = RenderSettings::new(window_width, window_height, surface_format, config);
        let layouts = BindGroupLayouts::new(device);
        let shaders = Shaders::new(config, device);
        let targets = RenderTargets::new(&settings, &layouts, device);

        let globals = RenderGlobals::new(&layouts, device);
        let objects = RenderObjects::new(&settings);
        let materials = RenderMaterials::new(&settings, &layouts, device, queue);

        let blit = BlitRenderManager::new(&layouts, &shaders, device);
        let pre_pass = PrePassRenderManager::new(&layouts, &shaders, device);
        let shadow = ShadowRenderManager::new(&settings, &layouts, &shaders, device);
        let sky = SkyRenderManager::new(&settings, &layouts, &shaders, device);
        let post_process = PostProcessRenderManager::new(&settings, &layouts, &shaders, device);
        let egui = EguiRenderManager::new(surface_format, device);
        Self {
            settings,
            layouts,
            shaders,
            targets,
            globals,
            objects,
            materials,
            pre_pass,
            shadow,
            sky,
            post_process,
            blit,
            egui,
        }
    }

    pub fn resize(
        &mut self,
        window_width: u32,
        window_height: u32,
        device: &Device,
        queue: &Queue,
    ) {
        self.settings.window_width = window_width;
        self.settings.window_height = window_height;

        if self.settings.auto_resolution {
            self.settings.resolution_width = window_width;
            self.settings.resolution_height = window_height;

            self.targets
                .resize(window_width, window_height, &self.layouts, device);

            self.post_process.resize(window_width, window_height, queue)
        }
    }
}
