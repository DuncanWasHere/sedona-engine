use crate::GameResources;
use crate::events::KeyEvent;
use crate::world::World;
use sedona_ecs::create_event_structs;
use sedona_event::{EventHandlers, EventQueue};

create_event_structs!(
    pub GameEvent,
    World,
    GameResources,
    {
        key_down: KeyEvent,
        key_up: KeyEvent,
    }
);
