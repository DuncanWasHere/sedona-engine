use crate::components::{StringId, WeatherComponent};
use sedona_ecs::entity;

#[entity]
pub struct WeatherEntity {
    pub string_id: StringId,
    pub weather_data: WeatherComponent,
}
