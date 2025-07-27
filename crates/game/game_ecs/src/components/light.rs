use glam::{Vec3, vec3};
use sedona_ecs::component;
use serde::{Deserialize, Serialize};

#[component]
pub struct LightData {
    pub color: Vec3,
    pub strength: f32,
    pub range: f32,
    pub light_type: LightType,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub enum LightType {
    #[default]
    PointLight,

    SpotLight {
        inner_cone_angle: f32,
        outer_cone_angle: f32,
    },
}

impl LightData {
    pub const WHITE: Self = Self {
        color: Vec3::ONE,
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const RED: Self = Self {
        color: vec3(1.0, 0.0, 0.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const GREEN: Self = Self {
        color: vec3(0.0, 1.0, 0.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const BLUE: Self = Self {
        color: vec3(0.0, 0.0, 1.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const YELLOW: Self = Self {
        color: vec3(1.0, 1.0, 0.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const TEAL: Self = Self {
        color: vec3(0.0, 1.0, 1.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };

    pub const PURPLE: Self = Self {
        color: vec3(1.0, 0.0, 1.0),
        strength: 1.0,
        range: 5.0,
        light_type: LightType::PointLight,
    };
}
