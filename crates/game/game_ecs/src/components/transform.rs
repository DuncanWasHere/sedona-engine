use glam::{EulerRot, Mat4, Quat, Vec3};
use sedona_ecs::component;

#[component]
pub struct TransformComponent {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: f32,
    pub dirty: bool,
}

impl TransformComponent {
    pub fn new(position: Option<Vec3>, rotation: Option<Vec3>, scale: Option<f32>) -> Self {
        Self {
            position: position.unwrap_or_default(),
            rotation: rotation.unwrap_or_default(),
            scale: scale.unwrap_or(1.0),
            dirty: false,
        }
    }

    pub fn with_position(position: Vec3) -> TransformComponent {
        Self {
            position,
            scale: 1.0,
            ..Default::default()
        }
    }

    pub fn with_rotation(rotation: Vec3) -> TransformComponent {
        Self {
            rotation,
            scale: 1.0,
            ..Default::default()
        }
    }

    pub fn with_scale(scale: f32) -> TransformComponent {
        Self {
            scale,
            ..Default::default()
        }
    }

    pub fn with_position_rotation(position: Vec3, rotation: Vec3) -> TransformComponent {
        Self {
            position,
            rotation,
            scale: 1.0,
            ..Default::default()
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        let scale = Vec3::splat(self.scale);

        let (pitch, yaw, roll) = (
            self.rotation[0].to_radians(),
            self.rotation[1].to_radians(),
            self.rotation[2].to_radians(),
        );

        let rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
        Mat4::from_scale_rotation_translation(scale, rotation, self.position)
    }
}
