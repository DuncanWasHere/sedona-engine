use crate::components::CameraComponent;
use sedona_ecs::component;

#[component]
pub struct NodeCameraComponent {
    pub camera: Option<CameraComponent>,
}
