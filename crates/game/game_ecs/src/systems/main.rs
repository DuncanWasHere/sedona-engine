use crate::components::{ModelPath, NodeEntityRef, PlayerComponent, StringId, TransformComponent};
use crate::entities::{PlayerEntity, PropEntity};
use crate::world::{World, WorldCreate};
use crate::{GameEventHandlers, GameResources};
use game_settings::{ACTIVE_CAMERA, ACTIVE_PLAYER};
use sedona_ecs::system;
use sedona_settings::SettingsValue;

#[system(group=pre_startup)]
pub fn main_pre_startup(world: &mut World, event_handlers: &mut GameEventHandlers) {}

#[system(group=startup)]
pub fn main_startup(
    world: &mut World,
    resources: &mut GameResources,
    event_handlers: &mut GameEventHandlers,
) {
    // world.create(PropEntity {
    //     string_id: StringId(Some(String::from("prop_ground"))),
    //     transform: TransformComponent::new(None, None, None),
    //     model_path: ModelPath(String::from("assets/models/ground.glb")),
    //     model: NodeEntityRef::default(),
    // });
    // 
    // world.create(PropEntity {
    //     string_id: StringId(Some(String::from("prop_cube"))),
    //     transform: TransformComponent::with_position(glam::vec3(0.0, 5.0, 0.0)),
    //     model_path: ModelPath(String::from("assets/models/cube.glb")),
    //     model: NodeEntityRef::default(),
    // });
    // 
    // world.create(PropEntity {
    //     string_id: StringId(Some(String::from("prop_ball"))),
    //     transform: TransformComponent::with_position(glam::vec3(-5.0, 8.0, 6.0)),
    //     model_path: ModelPath(String::from("assets/models/ball.glb")),
    //     model: NodeEntityRef::default(),
    // });
    // 
    // world.create(PropEntity {
    //     string_id: StringId(Some(String::from("prop_monolith"))),
    //     transform: TransformComponent::with_position(glam::vec3(0.0, -15.0, -250.0)),
    //     model_path: ModelPath(String::from("assets/models/monolith.glb")),
    //     model: NodeEntityRef::default(),
    // });
    // 
    // let player = world.create(PlayerEntity {
    //     string_id: StringId(Some(String::from("player_default"))),
    //     player_data: PlayerComponent {
    //         speed_multiplier: 1.0,
    //         third_person: false,
    //     },
    //     transform: TransformComponent::with_position(glam::vec3(0.0, 15.0, 0.0)),
    //     model_path: ModelPath(String::from("assets/models/default/player.glb")),
    //     model: NodeEntityRef::default(),
    // });

    // resources
    //     .variables
    //     .insert(ACTIVE_PLAYER, SettingsValue::EntityId(player.id()));
}

#[system(group=post_startup)]
pub fn main_post_startup(
    world: &mut World,
    resources: &mut GameResources,
    event_handlers: &mut GameEventHandlers,
) {
}

#[system(group=first)]
pub fn main_first(world: &mut World, resources: &mut GameResources) {}

#[system(group=pre_update)]
pub fn main_pre_update(world: &mut World, resources: &mut GameResources) {}

#[system(group=update)]
pub fn main_update(world: &mut World, resources: &mut GameResources) {}

#[system(group=post_update)]
pub fn main_post_update(world: &mut World, resources: &mut GameResources) {}

#[system(group=last)]
pub fn main_last(world: &mut World, resources: &mut GameResources) {}

#[system(group=quit)]
pub fn main_quit(world: &mut World, resources: &mut GameResources) {}

#[system(group=tick_first)]
pub fn main_tick_first(world: &mut World, resources: &mut GameResources) {}

#[system(group=pre_tick)]
pub fn main_pre_tick(world: &mut World, resources: &mut GameResources) {}

#[system(group=tick)]
pub fn main_tick(world: &mut World, resources: &mut GameResources) {}

#[system(group=post_tick)]
pub fn main_post_tick(world: &mut World, resources: &mut GameResources) {}

#[system(group=tick_last)]
pub fn main_tick_last(world: &mut World, resources: &mut GameResources) {}

#[system(group=window_resized)]
pub fn main_window_resized(world: &mut World, resources: &mut GameResources) {}

#[system(group=mouse_wheel)]
pub fn main_mouse_wheel(world: &mut World, resources: &mut GameResources, delta: f32) {}
