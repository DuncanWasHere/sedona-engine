use glam::Mat4;
use sedona_ecs::component;
use serde::{Deserialize, Serialize};

#[component]
pub struct CameraComponent {
    pub name: Option<String>,
    pub clip_near: f32,
    pub clip_far: f32,
    pub projection: CameraProjection,
    pub projection_matrix: Mat4,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum CameraProjection {
    Perspective { aspect_ratio: f32, fov_y: f32 },
    Orthographic { zoom_x: f32, zoom_y: f32 },
}

impl CameraComponent {
    pub fn new(
        name: Option<String>,
        clip_near: f32,
        clip_far: f32,
        projection: CameraProjection,
    ) -> Self {
        Self {
            name,
            clip_near,
            clip_far,
            projection,
            projection_matrix: Mat4::IDENTITY,
        }
    }
}

impl Default for CameraProjection {
    fn default() -> Self {
        Self::Perspective {
            aspect_ratio: 16.0 / 9.0,
            fov_y: 72.0_f32.to_radians(),
        }
    }
}
