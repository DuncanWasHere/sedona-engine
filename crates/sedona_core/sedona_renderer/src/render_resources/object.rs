use crate::render_resources::RenderSettings;
use crate::types::render_object::RenderObject;
use crate::types::{RenderNode, RendererError};
use glam::Vec3;
use slab::Slab;

pub struct RenderObjects {
    objects: Slab<RenderObject>,
    nodes: Slab<RenderNode>,
    translucent_objects: Vec<usize>,
    opaque_objects: Vec<usize>,
}

impl RenderObjects {
    pub fn new(settings: &RenderSettings) -> Self {
        let capacity = settings.initial_render_object_array_capacity;

        Self {
            objects: Slab::with_capacity(capacity),
            nodes: Slab::with_capacity(capacity),
            translucent_objects: Vec::with_capacity(capacity / 2),
            opaque_objects: Vec::with_capacity(capacity / 2),
        }
    }

    pub fn insert_object(&mut self, object: RenderObject, translucent: bool) -> usize {
        let index = self.objects.insert(object);

        if translucent {
            self.translucent_objects.push(index);
        } else {
            self.opaque_objects.push(index);
        }

        index
    }

    pub fn insert_node(&mut self, node: RenderNode) -> usize {
        self.nodes.insert(node)
    }

    pub fn remove_object(&mut self, index: usize) {
        self.objects.remove(index);

        self.opaque_objects.retain(|&i| i != index);
        self.translucent_objects.retain(|&i| i != index);
    }

    pub fn remove_node(&mut self, index: usize) {
        self.nodes.remove(index);
    }

    pub fn get_object(&self, index: usize) -> Result<&RenderObject, RendererError> {
        self.objects
            .get(index)
            .ok_or(RendererError::InvalidObjectIndex { index })
    }

    pub fn get_object_mut(&mut self, index: usize) -> Result<&mut RenderObject, RendererError> {
        self.objects
            .get_mut(index)
            .ok_or(RendererError::InvalidObjectIndex { index })
    }

    pub fn get_node(&self, index: usize) -> Result<&RenderNode, RendererError> {
        self.nodes
            .get(index)
            .ok_or(RendererError::InvalidNodeIndex { index })
    }

    pub fn get_node_mut(&mut self, index: usize) -> Result<&mut RenderNode, RendererError> {
        self.nodes
            .get_mut(index)
            .ok_or(RendererError::InvalidNodeIndex { index })
    }

    pub fn sort(&mut self, view_position: Vec3) {
        // Sort opaque front-to-back, grouped by material.
        self.opaque_objects.sort_unstable_by(|&a, &b| {
            let obj_a = &self.objects[a];
            let obj_b = &self.objects[b];

            let mat_cmp = obj_a.material.cmp(&obj_b.material);
            if mat_cmp != std::cmp::Ordering::Equal {
                return mat_cmp;
            }

            let node_a = &self.nodes[obj_a.node];
            let node_b = &self.nodes[obj_b.node];

            let pos_a = node_a
                .ubo
                .data()
                .model_matrix
                .to_scale_rotation_translation()
                .2;
            let pos_b = node_b
                .ubo
                .data()
                .model_matrix
                .to_scale_rotation_translation()
                .2;

            let dist_a = view_position.distance_squared(pos_a);
            let dist_b = view_position.distance_squared(pos_b);

            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Sort translucent back-to-front.
        self.translucent_objects.sort_unstable_by(|&a, &b| {
            let obj_a = &self.objects[a];
            let obj_b = &self.objects[b];

            let node_a = &self.nodes[obj_a.node];
            let node_b = &self.nodes[obj_b.node];

            let pos_a = node_a
                .ubo
                .data()
                .model_matrix
                .to_scale_rotation_translation()
                .2;
            let pos_b = node_b
                .ubo
                .data()
                .model_matrix
                .to_scale_rotation_translation()
                .2;

            let dist_a = view_position.distance_squared(pos_a);
            let dist_b = view_position.distance_squared(pos_b);

            dist_b
                .partial_cmp(&dist_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn clear(&mut self) {
        self.objects.clear();
        self.nodes.clear();
        self.opaque_objects.clear();
        self.translucent_objects.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &RenderObject> {
        self.opaque_objects
            .iter()
            .chain(self.translucent_objects.iter())
            .filter_map(move |&index| self.objects.get(index))
    }

    pub fn opaque_objects(&self) -> Vec<&RenderObject> {
        self.opaque_objects
            .iter()
            .filter_map(|&index| self.objects.get(index))
            .collect()
    }

    pub fn translucent_objects(&self) -> Vec<&RenderObject> {
        self.translucent_objects
            .iter()
            .filter_map(|&index| self.objects.get(index))
            .collect()
    }
}
