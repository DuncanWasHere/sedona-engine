use sedona_ecs::component;

#[component]
pub struct EnvironmentComponent {
    pub moon_textures: [String; 8],
    pub star_texture: String,
    pub sun_size: f32,
    pub moon_size: f32,
}
