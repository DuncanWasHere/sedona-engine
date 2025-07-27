use crate::events::KeyEvent;
use crate::world::World;
use crate::{GameEventHandlers, GameResources};
use sedona_app::KeyCode;
use sedona_ecs::system;

#[system(group=startup)]
pub fn window_startup(event_handlers: &mut GameEventHandlers) {
    event_handlers
        .key_down
        .register(KeyCode::F11, toggle_fullscreen);
}

fn toggle_fullscreen(_event: &KeyEvent, _world: &mut World, resources: &mut GameResources) {
    if resources.window.fullscreen() {
        resources.window.set_windowed();
    } else {
        resources.window.set_fullscreen();
    }
}
