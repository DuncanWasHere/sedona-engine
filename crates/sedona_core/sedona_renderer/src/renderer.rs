use crate::render_passes::*;
use crate::render_resources::*;
use crate::types::{
    MaterialUniforms, Pixels, RenderNode, RenderObject, RendererError, ShaderFlags, Vertex,
    VertexBufferObject,
};
use glam::{Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use sedona_settings::Settings;
use sedona_window::window::WindowContext;
use std::cell::RefCell;
use wgpu::*;

pub struct Renderer {
    surface: Surface<'static>,
    surface_format: TextureFormat,
    pub device: RefCell<Device>,
    pub queue: RefCell<Queue>,
    pub resources: RenderResources,
    swap_chain_interrupted: bool,
}

impl Renderer {
    pub fn new(window: &WindowContext, config: &Settings) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());

        let adapter = pollster::block_on(async {
            instance
                .request_adapter(&RequestAdapterOptions::default())
                .await
                .unwrap()
        });

        let surface = instance.create_surface(window.window().clone()).unwrap();
        let surface_format = surface.get_capabilities(&adapter).formats[0];

        let (device, queue) = pollster::block_on(async {
            adapter
                .request_device(&DeviceDescriptor {
                    required_limits: Limits {
                        max_bind_groups: 5,
                        ..Default::default()
                    },
                    required_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    ..Default::default()
                })
                .await
                .unwrap()
        });

        let (width, height) = window.size();

        let resources =
            RenderResources::new(width, height, config, surface_format, &device, &queue);

        let renderer = Self {
            surface,
            surface_format,
            device: RefCell::new(device),
            queue: RefCell::new(queue),
            resources,
            swap_chain_interrupted: false,
        };

        renderer.configure_surface(width, height);

        renderer
    }

    pub fn render(&mut self, window: &WindowContext) {
        if self.swap_chain_interrupted {
            let (width, height) = window.size();
            self.configure_surface(width, height);
        }

        let surface_texture = match self.surface.get_current_texture() {
            Ok(surface_texture) => surface_texture,
            Err(error) => {
                log::warn!("Could not get surface texture: {:?}", error);
                self.swap_chain_interrupted = true;
                return;
            }
        };

        let surface_view = surface_texture.texture.create_view(&TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
            ..Default::default()
        });

        let mut encoder = self
            .device
            .borrow()
            .create_command_encoder(&Default::default());

        let opaque_objects = &self.resources.objects.opaque_objects();

        render_pre_pass(opaque_objects, &mut encoder, &self.resources);

        render_shadow_pass(opaque_objects, &mut encoder, &self.resources);

        render_sky_pass(&mut encoder, &self.resources);

        render_object_pass(&mut encoder, &self.resources);

        render_bloom_extract_pass(&mut encoder, &self.resources);

        self.resources.post_process.buffers.blur_ubo.write_field(
            "direction",
            &Vec2::new(1.0, 0.0),
            &self.queue.borrow(),
        );
        render_bloom_blur_pass(&mut encoder, &self.resources);

        self.resources.post_process.buffers.blur_ubo.write_field(
            "direction",
            &Vec2::new(0.0, 1.0),
            &self.queue.borrow(),
        );
        render_bloom_blur_pass(&mut encoder, &self.resources);

        render_bloom_composite_pass(&mut encoder, &self.resources);

        compute_average_luminance(&mut encoder, &self.resources);

        compute_adapted_luminance(&mut encoder, &self.resources);

        render_tone_map_pass(&mut encoder, &surface_view, &self.resources);

        render_interface_pass(&mut encoder, &self.resources);

        self.queue.borrow().submit([encoder.finish()]);
        window.pre_present_notify();
        surface_texture.present();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.configure_surface(width, height);
        self.resources
            .resize(width, height, &self.device.borrow(), &self.queue.borrow());
    }

    fn configure_surface(&self, width: u32, height: u32) {
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width,
            height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        self.surface
            .configure(&self.device.borrow(), &surface_config);
    }

    pub fn create_render_object<V: Vertex>(
        &mut self,
        vertices: &[V],
        indices: &[u32],
        material: usize,
        node: usize,
    ) -> Result<usize, RendererError> {
        let vbo = VertexBufferObject::new(vertices, indices, &self.device.borrow());
        let object = RenderObject::new(vbo, material, node);

        let material = self.resources.materials.get_material(material)?;
        let translucent = material.shader_key.flags.contains(ShaderFlags::BLEND_ALPHA);

        Ok(self.resources.objects.insert_object(object, translucent))
    }

    pub fn create_render_material(
        &mut self,
        uniform_data: MaterialUniforms,
        textures: &[Option<u64>; 5],
        shader_path: &str,
        shader_flags: ShaderFlags,
    ) -> Result<usize, RendererError> {
        self.resources.materials.insert_material(
            uniform_data,
            textures,
            shader_path,
            shader_flags,
            &self.resources.layouts,
            &self.device.borrow(),
        )
    }

    pub fn create_render_node(&mut self, transform: Mat4) -> usize {
        let node = RenderNode::new(transform, &self.resources, &self.device.borrow());
        self.resources.objects.insert_node(node)
    }

    pub fn create_material_texture(&mut self, pixels: impl Pixels) -> u64 {
        self.resources
            .materials
            .create_texture(pixels, &self.device.borrow(), &self.queue.borrow())
    }
}
