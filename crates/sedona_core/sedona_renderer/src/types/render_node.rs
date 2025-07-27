use crate::render_resources::RenderResources;
use crate::types::{ModelUniforms, UniformBufferObject};
use glam::{Mat3, Mat4};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, Device, Queue};

#[derive(Clone, Debug)]
pub struct RenderNode {
    pub ubo: UniformBufferObject<ModelUniforms>,
    pub bind_group: BindGroup,
}

impl RenderNode {
    pub fn new(model_matrix: Mat4, resources: &RenderResources, device: &Device) -> RenderNode {
        let model_normal_matrix =
            Mat4::from_mat3(Mat3::from_mat4(model_matrix).inverse().transpose());

        let uniform_data = ModelUniforms {
            model_matrix,
            model_normal_matrix,
        };

        let ubo = UniformBufferObject::with_data(uniform_data, device);

        let bind_group_layout = resources.layouts.model();
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("bind_group_model"),
            layout: bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: ubo.as_entire_binding(),
            }],
        });

        Self { ubo, bind_group }
    }

    pub fn update_model_matrix(&mut self, model_matrix: Mat4, queue: &Queue) {
        let model_normal_matrix =
            Mat4::from_mat3(Mat3::from_mat4(model_matrix).inverse().transpose());

        let uniform_data = ModelUniforms {
            model_matrix,
            model_normal_matrix,
        };

        self.ubo.set(uniform_data, queue);
    }
}
