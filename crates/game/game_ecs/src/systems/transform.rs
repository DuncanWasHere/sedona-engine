use crate::GameResources;
use crate::components::{Children, NodeComponent, NodeEntityRef, TransformComponent};
use crate::world::{Entity, Query, World};
use glam::Mat4;
use sedona_ecs::system;

#[system(group=update)]
pub fn transform_update(
    world: &mut World,
    resources: &mut GameResources,
    world_entities: Query<(&TransformComponent, &NodeEntityRef)>,
    node_entities: Query<(&mut NodeComponent, &mut Children)>,
) {
    let dirty_roots: Vec<(Mat4, Entity)> = world
        .with_query(world_entities)
        .iter()
        .filter_map(|(transform, node_ref)| {
            if transform.dirty {
                node_ref.0.map(|e| (transform.to_matrix(), e))
            } else {
                None
            }
        })
        .collect();

    for (root_matrix, root_entity) in dirty_roots {
        propagate_transform(world, root_entity, &root_matrix, resources, node_entities);
    }
}

fn propagate_transform(
    world: &mut World,
    node_entity: Entity,
    parent_transform: &Mat4,
    resources: &mut GameResources,
    node_entities: Query<(&mut NodeComponent, &mut Children)>,
) {
    let (global_transform, children_entities) = {
        if let Some((node, children)) = world.with_query_mut(node_entities).get_mut(node_entity.id()) {
            node.update_global(parent_transform);
            let queue = &resources.renderer.queue.borrow();
            let render_node = resources
                .renderer
                .resources
                .objects
                .get_node_mut(node.handle)
                .unwrap();
            render_node.update_model_matrix(node.global_transform, queue);

            let children_entities: Vec<Entity> = children.0.iter().filter_map(|c| c.0).collect();

            (node.global_transform, children_entities)
        } else {
            return;
        }
    };

    for child in children_entities {
        propagate_transform(world, child, &global_transform, resources, node_entities);
    }
}
