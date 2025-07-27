use crate::GameResources;
use crate::components::{ModelPath, NodeEntityRef, TransformComponent};
use crate::utils::gltf::load_gltf_scene;
use crate::world::{Query, World};
use sedona_ecs::system;

#[system(group=post_startup)]
pub fn asset_load_models(
    world: &mut World,
    resources: &mut GameResources,
    renderables: Query<(&mut ModelPath, &mut TransformComponent, &mut NodeEntityRef)>,
) {
    let mut load_requests = Vec::new();

    for idx in 0..world.with_query_mut(renderables).len() {
        if let Some((model_path, transform, _)) = world.with_query_mut(renderables).at_mut(idx) {
            load_requests.push((idx, model_path.0.clone(), transform.to_matrix()));
        }
    }

    for (idx, path, matrix) in load_requests {
        match load_gltf_scene(&path, matrix, world, resources) {
            Ok(root_entity) => {
                if let Some((_, _, node_ref)) = world.with_query_mut(renderables).at_mut(idx) {
                    node_ref.0 = Some(root_entity);
                }
            }
            Err(err) => {
                log::error!("Failed to load model '{path}': {err}");
            }
        }
    }
}
