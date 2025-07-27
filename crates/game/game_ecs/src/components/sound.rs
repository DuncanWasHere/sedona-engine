use sedona_ecs::component;

#[component]
pub struct SoundData {
    pub path: String,
    pub static_attenuation: f32,
    pub looping: bool,
}
