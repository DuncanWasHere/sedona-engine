use crate::world::Entity;
use sedona_ecs::component;

#[component]
pub struct CameraEntityRef(pub Option<Entity>);

#[component]
pub struct NodeEntityRef(pub Option<Entity>);

#[component]
pub struct ChildEntityRef(pub Option<Entity>);

#[component]
pub struct WeatherEntityRef(pub Option<Entity>);

#[component]
pub struct SoundEntityRef(pub Option<Entity>);
