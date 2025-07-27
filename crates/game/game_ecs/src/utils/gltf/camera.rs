use crate::components::{CameraComponent, CameraProjection, NodeCameraComponent};
use gltf::Camera;
use gltf::camera::Projection;

pub(crate) fn process_camera(g_camera: &Camera) -> NodeCameraComponent {
    let mut camera_component = CameraComponent::default();

    match g_camera.projection() {
        Projection::Perspective(perspective) => {
            camera_component.clip_near = perspective.znear();
            camera_component.clip_far = perspective.zfar().unwrap_or_default();

            camera_component.projection = CameraProjection::Perspective {
                aspect_ratio: perspective.aspect_ratio().unwrap_or_default(),
                fov_y: perspective.yfov(),
            };
        }

        Projection::Orthographic(orthographic) => {
            camera_component.clip_near = orthographic.znear();
            camera_component.clip_far = orthographic.zfar();

            camera_component.projection = CameraProjection::Orthographic {
                zoom_x: orthographic.xmag(),
                zoom_y: orthographic.ymag(),
            };
        }
    };

    if let Some(name) = g_camera.name() {
        camera_component.name = Some(name.to_owned());
    }

    NodeCameraComponent {
        camera: Some(camera_component),
    }
}
