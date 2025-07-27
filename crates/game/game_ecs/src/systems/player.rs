use crate::world::EntityType;
use crate::components::{Children, NodeCameraComponent, NodeComponent, NodeEntityRef};
use crate::components::{PlayerComponent, TransformComponent};
use crate::events::KeyEvent;
use crate::systems::{camera_window_resized, update_projection_matrix};
use crate::world::{Entity, Query, QueryMutFrom, World};
use crate::{GameEventHandlers, GameResources};
use game_settings::{ACTIVE_CAMERA, ACTIVE_PLAYER, MOUSE_SENSITIVITY};
use glam::{EulerRot, Quat, Vec3};
use sedona_app::KeyCode;
use sedona_ecs::{system, Uuid};
use sedona_settings::{SettingsValue, value_as};

#[system(group=pre_startup)]
pub fn player_pre_startup(world: &mut World, event_handlers: &mut GameEventHandlers) {
    event_handlers
        .key_down
        .register(KeyCode::KeyF, toggle_perspective);
}

#[system(group=post_startup)]
pub fn player_post_startup(
    world: &mut World,
    resources: &mut GameResources,
    players: Query<(&Entity, &PlayerComponent)>,
) {
    if let Some((entity, _)) = world.with_query(players).iter().next() {
        resources
            .variables
            .insert(ACTIVE_PLAYER, SettingsValue::EntityId(entity.id()));
    }
}

#[system(group=update)]
pub fn player_update(
    world: &mut World,
    resources: &mut GameResources,
    players: Query<(&mut PlayerComponent, &mut TransformComponent)>,
    _pq0: Query<&mut PlayerComponent>,
) {
    let active_player_id = match resources.variables.get(ACTIVE_PLAYER) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    if let Some((player, transform)) = world.with_query_mut(players).get_mut(active_player_id) {
        let mouse_sensitivity = value_as(resources.config.get(MOUSE_SENSITIVITY)).unwrap_or(0.1);

        let (orig_pos, orig_rot) = (transform.position, transform.rotation);

        // Mouse look
        let (delta_x, delta_y) = resources.input_state.mouse_delta();
        let mut pitch = orig_rot[0] - delta_y * mouse_sensitivity;
        let yaw = orig_rot[1] - delta_x * mouse_sensitivity;
        pitch = pitch.clamp(-89.0, 89.0);
        transform.rotation = Vec3::new(pitch, yaw, orig_rot[2]);

        // Movement
        let mut direction = Vec3::ZERO;
        if resources.input_state.keys_held().contains(&KeyCode::KeyW) {
            direction.z += 1.0;
        }
        if resources.input_state.keys_held().contains(&KeyCode::KeyS) {
            direction.z -= 1.0;
        }
        if resources.input_state.keys_held().contains(&KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if resources.input_state.keys_held().contains(&KeyCode::KeyD) {
            direction.x += 1.0;
        }

        let pitch_rad = transform.rotation[0].to_radians();
        let yaw_rad = transform.rotation[1].to_radians();
        let roll_rad = transform.rotation[2].to_radians();

        let rotation_quat = Quat::from_euler(EulerRot::YXZ, yaw_rad, pitch_rad, roll_rad);

        if direction.length_squared() != 0.0 {
            let direction = direction.normalize();

            let forward = rotation_quat * Vec3::NEG_Z;
            let right = rotation_quat * Vec3::X;

            let move_vec = forward * direction.z + right * direction.x;
            let speed = player.speed_multiplier * 5.0;
            let distance = speed * resources.input_state.dt();
            let new_pos = Vec3::from(orig_pos) + move_vec * distance;
            transform.position = new_pos;
        }

        transform.dirty = true;
    }
}

#[system(group=mouse_wheel)]
pub fn mod_player_speed(
    delta: f32,
    world: &mut World,
    resources: &GameResources,
    players: Query<&mut PlayerComponent>,
) {
    const MIN_SPEED: f32 = 0.001;
    const MAX_SPEED: f32 = 10000.0;
    const STEP: f32 = 0.1;

    let active_player_id = match resources.variables.get(ACTIVE_PLAYER) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    if let Some(player) = world.with_query_mut(players).get_mut(active_player_id) {
        let current = player.speed_multiplier.max(MIN_SPEED);

        let log_speed = current.log10();

        let new_log_speed = (log_speed + delta * STEP).clamp(MIN_SPEED.log10(), MAX_SPEED.log10());
        let new_speed = 10.0_f32.powf(new_log_speed);

        player.speed_multiplier = new_speed;
    }
}

#[system(group=manual_event)]
pub fn _toggle_perspective_query(
    world: &mut World,
    resources: &mut GameResources,
    children: Query<&Children>,
    players: Query<(&mut PlayerComponent, &mut NodeEntityRef)>,
    node_info: Query<(&Entity, &NodeComponent, &NodeCameraComponent)>,
) {
}

pub fn toggle_perspective(_event: &KeyEvent, world: &mut World, resources: &mut GameResources) {
    let players: Query<(&mut PlayerComponent, &mut NodeEntityRef)> = Query::new();
    let node_info: Query<(&Entity, &NodeComponent, &NodeCameraComponent)> = Query::new();
    let children: Query<&Children> = Query::new();

    let active_player_id = match resources.variables.get(ACTIVE_PLAYER) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    let maybe_target = {
        let mut players_query = world.with_query_mut(players);
        if let Some((player, model)) = players_query.get_mut(active_player_id) {
            player.third_person = !player.third_person;

            let target_name = if player.third_person {
                "camera_third_person"
            } else {
                "camera_first_person"
            };

            model.0.map(|model_entity| (target_name, model_entity))
        } else {
            None
        }
    };

    if let Some((target_name, model_entity)) = maybe_target {
        if let Some(id) = find_camera(target_name, model_entity, world, node_info, children) {
            resources
                .variables
                .insert(ACTIVE_CAMERA, SettingsValue::EntityId(id));
            camera_window_resized(world, resources, Query::new(), Query::new());
        }
    }
}

fn find_camera(
    target_name: &str,
    start_node: Entity,
    world: &mut World,
    node_info: Query<(&Entity, &NodeComponent, &NodeCameraComponent)>,
    children: Query<(&Children)>,
) -> Option<Uuid> {
    let mut stack = vec![start_node];

    while let Some(current_entity) = stack.pop() {
        if let Some((entity, node, cam)) = world.with_query(node_info).get(current_entity.id()) {
            if let Some(name) = &node.name {
                if name == target_name {
                    return Some(entity.id());
                }
            }
        }

        if let Some(children) = world.with_query(children).get(current_entity.id()) {
            for child in children.0.iter() {
                if let Some(entity) = child.0 {
                    stack.push(entity);
                }
            }
        }
    }

    log::error!("Could not find a camera with name {target_name}");
    None
}
