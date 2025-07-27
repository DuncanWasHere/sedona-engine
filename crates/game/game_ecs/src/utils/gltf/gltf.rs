use crate::GameResources;
use crate::components::{
    ChildEntityRef, Children, MeshComponents, NodeCameraComponent, NodeComponent,
};
use crate::entities::NodeEntity;
use crate::utils::gltf::{process_camera, process_meshes};
use crate::world::{Entity, World, WorldCreate};
use glam::{Mat4, Quat, Vec3};
use gltf::import;
use std::error::Error;
use std::path::Path;

pub fn load_gltf_scene<P: AsRef<Path>>(
    path: P,
    transform: Mat4,
    world: &mut World,
    resources: &mut GameResources,
) -> Result<Entity, Box<dyn Error>> {
    // glTF types are prefixed with g_ to distinguish them from game types.

    let (g_document, g_buffers, g_images) = import(&path)?;
    let g_scene = g_document
        .default_scene()
        .or_else(|| g_document.scenes().next())
        .ok_or("No scene in glTF")?;

    let node_handle = resources.renderer.create_render_node(transform);
    let node_component =
        NodeComponent::new(node_handle, Some(String::from("scene_root")), Mat4::IDENTITY);

    let mut children = Vec::new();
    for g_node in g_scene.nodes() {
        children.push(process_node_tree(
            &g_node, &g_buffers, &g_images, transform, world, resources,
        ));
    }

    let node = world.create(NodeEntity {
        node: node_component,
        meshes: MeshComponents::default(),
        camera: NodeCameraComponent::default(),
        children: Children(children),
    });

    Ok(node)
}

fn process_node_tree(
    g_node: &gltf::Node,
    g_buffers: &[gltf::buffer::Data],
    g_images: &[gltf::image::Data],
    parent_transform: Mat4,
    world: &mut World,
    resources: &mut GameResources,
) -> ChildEntityRef {
    let local_transform = match g_node.transform() {
        gltf::scene::Transform::Matrix { matrix } => Mat4::from_cols_array_2d(&matrix),
        gltf::scene::Transform::Decomposed {
            translation,
            rotation,
            scale,
        } => {
            let translation = Vec3::from(translation);
            let rotation = Quat::from_array(rotation);
            let scale = Vec3::from(scale);

            Mat4::from_scale_rotation_translation(scale, rotation, translation)
        }
    };

    let global_transform = local_transform * parent_transform;

    let node_handle = resources.renderer.create_render_node(global_transform);
    let name = g_node.name().map(str::to_string);
    let node_component =
        NodeComponent::with_global_transform(node_handle, name, local_transform, global_transform);

    let mut children = Vec::new();
    for child in g_node.children() {
        children.push(process_node_tree(
            &child,
            g_buffers,
            g_images,
            global_transform,
            world,
            resources,
        ));
    }

    let mut meshes = Vec::new();
    if let Some(g_mesh) = g_node.mesh() {
        meshes = process_meshes(
            g_mesh.primitives(),
            g_buffers,
            g_images,
            node_handle,
            resources,
        );
    }

    let camera = {
        if let Some(g_camera) = g_node.camera() {
            process_camera(&g_camera)
        } else {
            NodeCameraComponent::default()
        }
    };

    let node = world.create(NodeEntity {
        node: node_component,
        meshes: MeshComponents(meshes),
        camera,
        children: Children(children),
    });

    ChildEntityRef(Some(node))
}
