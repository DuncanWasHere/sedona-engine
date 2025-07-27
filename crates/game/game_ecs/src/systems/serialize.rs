use crate::events::KeyEvent;
use crate::utils::serialize::save_game_package;
use crate::world::World;
use crate::{GameEventHandlers, GameResources};
use sedona_app::KeyCode;
use sedona_ecs::system;

#[system(group=pre_startup)]
pub fn serialize_pre_startup(world: &mut World, event_handlers: &mut GameEventHandlers) {
    event_handlers.key_down.register(KeyCode::KeyS, save_world);
}

pub fn save_world(_event: &KeyEvent, world: &mut World, resources: &mut GameResources) {
    if resources
        .input_state
        .keys_held
        .contains(&KeyCode::ControlLeft)
    {
        save_game_package(world, "test.ron");
    }
}
