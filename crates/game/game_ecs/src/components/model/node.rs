use glam::Mat4;
use sedona_ecs::component;

#[component]
pub struct NodeComponent {
    pub handle: usize,
    pub name: Option<String>,
    pub global_transform: Mat4,
    pub local_transform: Mat4,
}

impl NodeComponent {
    pub fn new(handle: usize, name: Option<String>, local_transform: Mat4) -> Self {
        Self {
            handle,
            name,
            global_transform: local_transform,
            local_transform,
        }
    }

    pub fn with_global_transform(
        handle: usize,
        name: Option<String>,
        local_transform: Mat4,
        global_transform: Mat4,
    ) -> Self {
        Self {
            handle,
            name,
            global_transform,
            local_transform,
        }
    }

    pub fn with_parent_transform(
        handle: usize,
        name: Option<String>,
        local_transform: Mat4,
        parent_transform: &Mat4,
    ) -> Self {
        Self {
            handle,
            name,
            global_transform: *parent_transform * local_transform,
            local_transform,
        }
    }

    pub fn update_global(&mut self, parent_transform: &Mat4) {
        self.global_transform = *parent_transform * self.local_transform;
    }
}
