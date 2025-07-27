use sedona_ecs::component;

#[component]
pub struct PlayerComponent {
    pub third_person: bool,
    pub speed_multiplier: f32,
}
