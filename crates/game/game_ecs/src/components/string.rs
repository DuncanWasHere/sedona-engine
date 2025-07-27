use sedona_ecs::component;

#[component]
pub struct StringId(pub Option<String>);

#[component]
pub struct Name(pub String);

#[component]
pub struct ModelPath(pub String);
