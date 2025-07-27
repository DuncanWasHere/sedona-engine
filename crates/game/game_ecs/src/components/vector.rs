use crate::components::{CameraEntityRef, ChildEntityRef, MeshComponent, WeatherEntityRef};
use sedona_ecs::component;

#[component]
pub struct Children(pub Vec<ChildEntityRef>);

#[component]
pub struct Cameras(pub Vec<CameraEntityRef>);

#[component]
pub struct MeshComponents(pub Vec<MeshComponent>);

#[component]
pub struct Weathers(pub Vec<WeatherEntityRef>);
