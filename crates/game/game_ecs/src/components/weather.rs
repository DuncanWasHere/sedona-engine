use glam::{Vec3, Vec4};
use sedona_ecs::component;
use serde::{Deserialize, Serialize};

#[component]
pub struct WeatherComponent {
    pub midnight_lighting: WeatherLightingProfile,
    pub night_lighting: WeatherLightingProfile,
    pub dawn_lighting: WeatherLightingProfile,
    pub sunrise_lighting: WeatherLightingProfile,
    pub day_lighting: WeatherLightingProfile,
    pub noon_lighting: WeatherLightingProfile,
    pub sunset_lighting: WeatherLightingProfile,
    pub dusk_lighting: WeatherLightingProfile,
    pub cloud_texture: String,
    pub rayleigh: Vec3,
    pub mie: Vec3,
    pub mie_anisotropy: f32,
    pub rayleigh_factor: f32,
    pub mie_factor: f32,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct WeatherLightingProfile {
    pub ambient_color: Vec3,
    pub ambient_strength: f32,
    pub fog_color: Vec3,
    pub fog_exponent: f32,
    pub fog_start: f32,
    pub fog_end: f32,
    pub sunlight_color: Vec3,
    pub sunlight_strength: f32,
    pub horizon_color: Vec3,
    pub sky_lower_color: Vec3,
    pub sky_upper_color: Vec3,
    pub cloud_layer_color: Vec3,
    pub stars_color: Vec4,
    pub moon_color: Vec4,
    pub sun_color: Vec3,
    pub turbidity: f32,
}
