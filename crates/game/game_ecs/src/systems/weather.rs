use crate::world::EntityType;
use crate::GameResources;
use crate::components::{
    EnvironmentComponent, StringId, WeatherComponent, WeatherEntityRef, WeatherLightingProfile,
    Weathers,
};
use crate::entities::{EnvironmentEntity, WeatherEntity};
use crate::utils::conditions::is_night;
use crate::utils::constants::*;
use crate::utils::interpolation::*;
use crate::world::{Entity, Query, World, WorldCreate};
use game_settings::{ACTIVE_ENVIORNMENT, ACTIVE_WEATHER, MOON_DIRECTION, MOON_PHASE, MOON_ROTATION, SOLAR_TIME, STAR_ROTATION, SUN_DIRECTION, SUN_ROTATION};
use glam::{Quat, Vec3, Vec4};
use sedona_ecs::system;
use sedona_renderer::types::{LightingUniforms, SkyGradientUniforms, SkyMode, SkyPbrUniforms};
use sedona_settings::{SettingsValue, value_as};

#[system(group=startup)]
pub fn weather_startup(world: &mut World, resources: &mut GameResources) {
    let weather_data = WeatherComponent {
        midnight_lighting: profile(
            [0.003, 0.003, 0.006],
            0.04,
            [0.02, 0.02, 0.03],
            [0.01, 0.01, 0.012],
            0.55,
            650.0,
            2300.0,
            [0.001, 0.001, 0.006],
            [0.004, 0.004, 0.02],
            [0.008, 0.008, 0.025],
            [0.8, 0.8, 0.9, 1.0],
            [10.0, 10.0, 10.0],
            [0.02, 0.02, 0.03],
            0.18,
            [0.55, 0.6, 0.75, 1.0],
            2.0,
        ),

        night_lighting: profile(
            [0.008, 0.008, 0.015],
            0.08,
            [0.04, 0.04, 0.06],
            [0.02, 0.02, 0.022],
            0.65,
            600.0,
            2200.0,
            [0.003, 0.004, 0.01],
            [0.015, 0.015, 0.035],
            [0.025, 0.025, 0.05],
            [0.75, 0.75, 0.85, 1.0],
            [10.0, 10.0, 10.0],
            [0.03, 0.03, 0.05],
            0.22,
            [0.6, 0.65, 0.8, 1.0],
            2.0,
        ),

        dawn_lighting: profile(
            [0.012, 0.01, 0.018],
            0.1,
            [0.08, 0.07, 0.065],
            [0.05, 0.05, 0.145],
            0.75,
            550.0,
            2100.0,
            [0.02, 0.015, 0.025],
            [0.025, 0.02, 0.04],
            [0.035, 0.03, 0.06],
            [0.4, 0.45, 0.6, 1.0],
            [10.0, 10.0, 10.0],
            [0.04, 0.035, 0.06],
            0.24,
            [0.025, 0.02, 0.04, 1.0],
            2.5,
        ),

        sunrise_lighting: profile(
            [0.09, 0.06, 0.04],
            0.18,
            [0.32, 0.27, 0.22],
            [0.3, 0.24, 0.18],
            0.85,
            300.0,
            1450.0,
            [0.6, 0.3, 0.15],
            [0.75, 0.4, 0.2],
            [0.9, 0.5, 0.25],
            [0.02, 0.02, 0.05, 1.0],
            [10.0, 10.0, 10.0],
            [0.8, 0.6, 0.4],
            0.85,
            [0.75, 0.4, 0.2, 1.0],
            3.4,
        ),

        day_lighting: profile(
            [0.2, 0.25, 0.3],
            0.3,
            [0.6, 0.7, 0.8],
            [0.4, 0.5, 0.6],
            1.6,
            200.0,
            1400.0,
            [0.5, 0.55, 0.6],
            [0.4, 0.5, 0.75],
            [0.35, 0.45, 0.85],
            [0.0, 0.0, 0.0, 1.0],
            [10.0, 10.0, 10.0],
            [0.95, 1.0, 0.85],
            1.0,
            [0.35, 0.45, 0.85, 1.0],
            2.1,
        ),

        noon_lighting: profile(
            [0.32, 0.38, 0.42],
            0.42,
            [0.85, 0.95, 1.0],
            [0.7, 0.8, 0.9],
            1.6,
            150.0,
            1300.0,
            [0.75, 0.8, 0.85],
            [0.6, 0.75, 1.0],
            [0.55, 0.75, 1.2],
            [0.0, 0.0, 0.0, 1.0],
            [10.0, 10.0, 10.0],
            [1.2, 1.2, 1.1],
            1.25,
            [0.55, 0.75, 1.2, 1.0],
            2.0,
        ),

        sunset_lighting: profile(
            [0.16, 0.1, 0.06],
            0.22,
            [0.4, 0.3, 0.25],
            [0.62, 0.28, 0.05],
            1.45,
            150.0,
            1300.0,
            [0.65, 0.3, 0.1],
            [0.8, 0.35, 0.15],
            [0.95, 0.3, 0.1],
            [0.04, 0.04, 0.08, 1.0],
            [10.0, 10.0, 10.0],
            [0.75, 0.5, 0.3],
            0.9,
            [0.8, 0.35, 0.15, 1.0],
            4.8,
        ),

        dusk_lighting: profile(
            [0.028, 0.027, 0.03],
            0.1,
            [0.065, 0.065, 0.07],
            [0.033, 0.033, 0.038],
            1.05,
            350.0,
            1700.0,
            [0.035, 0.035, 0.04],
            [0.06, 0.06, 0.07],
            [0.09, 0.095, 0.11],
            [0.25, 0.3, 0.4, 1.0],
            [10.0, 10.0, 10.0],
            [0.16, 0.17, 0.18],
            0.35,
            [0.52, 0.55, 0.65, 0.0],
            3.3,
        ),

        cloud_texture: "assets/textures/sky/clouds.png".to_string(),
        rayleigh: glam::vec3(5.8e-6, 13.5e-6, 33.1e-6),
        mie: glam::vec3(0.001, 0.001, 0.001),
        mie_anisotropy: 0.76,
        rayleigh_factor: 6.0,
        mie_factor: 1.0,
    };

    let weather = world.create(WeatherEntity {
        string_id: StringId(Some(String::from("weather_default"))),
        weather_data,
    });

    resources
        .variables
        .insert(ACTIVE_WEATHER, SettingsValue::EntityId(weather.id()));

    let environment_data = EnvironmentComponent {
        moon_textures: [
            "assets/textures/sky/moon_new.png".to_string(),
            "assets/textures/sky/moon_waxing_crescent.png".to_string(),
            "assets/textures/sky/moon_first_quarter.png".to_string(),
            "assets/textures/sky/moon_waxing_gibbous.png".to_string(),
            "assets/textures/sky/moon_full.png".to_string(),
            "assets/textures/sky/moon_waning_gibbous.png".to_string(),
            "assets/textures/sky/moon_third_quarter.png".to_string(),
            "assets/textures/sky/moon_waning_crescent.png".to_string(),
        ],
        star_texture: "assets/textures/sky/stars.png".to_string(),
        sun_size: 0.01862,
        moon_size: 0.1,
    };

    let device = &resources.renderer.device.borrow();
    let queue = &resources.renderer.queue.borrow();
    let layouts = &resources.renderer.resources.layouts;
    resources
        .renderer
        .resources
        .sky
        .buffers
        .sun_ubo
        .write_field("size", &environment_data.sun_size, queue);
    resources
        .renderer
        .resources
        .sky
        .buffers
        .moon_ubo
        .write_field("size", &environment_data.moon_size, queue);
    resources.renderer.resources.sky.set_moon_textures(
        &environment_data.moon_textures,
        layouts,
        device,
        queue,
    );
    resources.renderer.resources.sky.set_star_map_texture(
        &environment_data.star_texture,
        layouts,
        device,
        queue,
    );

    let environment = world.create(EnvironmentEntity {
        string_id: StringId(Some(String::from("environment_default"))),
        environment_data,
        weathers: Weathers(vec![WeatherEntityRef(Some(weather))]),
    });
    resources.variables.insert(
        ACTIVE_ENVIORNMENT,
        SettingsValue::EntityId(environment.id()),
    );
}

#[system(group=post_tick)]
pub fn weather_tick(
    world: &mut World,
    resources: &mut GameResources,
    weathers: Query<&WeatherComponent>,
) {
    let active_weather_id = match resources.variables.get(ACTIVE_WEATHER) {
        Some(SettingsValue::EntityId(id)) => *id,
        _ => return,
    };

    let queue = &resources.renderer.queue.borrow();

    let time = value_as(resources.variables.get(SOLAR_TIME)).unwrap_or(0.0);

    if let Some(weather_data) = world.with_query(weathers).get(active_weather_id) {
        let lighting_profile = interpolate_weather_lighting_profiles(weather_data.clone(), time);
        let sun_rotation = match resources.variables.get(SUN_ROTATION) {
            Some(SettingsValue::Quat(value)) => *value,
            _ => Quat::IDENTITY,
        };

        let sun_direction = match resources.variables.get(SUN_DIRECTION) {
            Some(SettingsValue::Vec3(value)) => *value,
            _ => Vec3::ZERO,
        };

        let moon_rotation = match resources.variables.get(MOON_ROTATION) {
            Some(SettingsValue::Quat(value)) => *value,
            _ => Quat::IDENTITY,
        };

        let moon_direction = match resources.variables.get(MOON_DIRECTION) {
            Some(SettingsValue::Vec3(value)) => *value,
            _ => Vec3::ZERO,
        };

        let moon_phase = value_as(resources.variables.get(MOON_PHASE)).unwrap_or(0);

        let star_rotation = match resources.variables.get(STAR_ROTATION) {
            Some(SettingsValue::Quat(value)) => *value,
            _ => Quat::IDENTITY,
        };

        let directional_angle = if is_night(time) {
            moon_direction
        } else {
            sun_direction
        };

        let lighting_uniforms = LightingUniforms {
            directional_angle: directional_angle.extend(0.0),
            directional_color: lighting_profile.sunlight_color.extend(1.0),
            directional_strength: lighting_profile.sunlight_strength,
            ambient_color: lighting_profile.ambient_color.extend(1.0),
            ambient_strength: lighting_profile.ambient_strength,
            fog_color: lighting_profile.fog_color.extend(1.0),
            fog_exponent: lighting_profile.fog_exponent,
            fog_start: lighting_profile.fog_start,
            fog_end: lighting_profile.fog_end,
            contrast: 0.0,
            saturation: 0.0,
            gamma: 0.0,
        };

        resources
            .renderer
            .resources
            .globals
            .buffers
            .lighting_ubo
            .set(lighting_uniforms, queue);

        match resources.renderer.resources.sky.sky_mode {
            SkyMode::Gradient => {
                let sky_gradient_uniforms = SkyGradientUniforms {
                    horizon_color: lighting_profile.horizon_color.extend(1.0),
                    sky_upper_color: lighting_profile.sky_upper_color.extend(1.0),
                    sky_lower_color: lighting_profile.sky_lower_color.extend(1.0),
                    sun_direction,
                    mie: weather_data.mie,
                    mie_anisotropy: weather_data.mie_anisotropy,
                    turbidity: lighting_profile.turbidity,
                };

                resources
                    .renderer
                    .resources
                    .sky
                    .buffers
                    .sky_gradient_ubo
                    .set(sky_gradient_uniforms, queue);
            }
            SkyMode::Pbr => {
                let sky_pbr_uniforms = SkyPbrUniforms {
                    sun_direction: sun_direction.extend(0.0),
                    mie: weather_data.mie.extend(1.0),
                    mie_factor: weather_data.mie_factor,
                    mie_anisotropy: weather_data.mie_anisotropy,
                    turbidity: lighting_profile.turbidity,
                    rayleigh: weather_data.rayleigh.extend(1.0),
                    rayleigh_factor: weather_data.rayleigh_factor,
                };

                resources
                    .renderer
                    .resources
                    .sky
                    .buffers
                    .sky_pbr_ubo
                    .set(sky_pbr_uniforms, queue);
            }
            _ => {}
        }

        let sun_uniforms = resources.renderer.resources.sky.buffers.sun_ubo.data_mut();
        sun_uniforms.rotation = sun_rotation;
        sun_uniforms.tint = lighting_profile.sun_color.extend(1.0);
        resources
            .renderer
            .resources
            .sky
            .buffers
            .sun_ubo
            .write(queue);

        let moon_uniforms = resources.renderer.resources.sky.buffers.moon_ubo.data_mut();
        moon_uniforms.rotation = moon_rotation;
        moon_uniforms.phase_index = moon_phase;
        moon_uniforms.tint = lighting_profile.moon_color;
        resources
            .renderer
            .resources
            .sky
            .buffers
            .moon_ubo
            .write(queue);

        let star_uniforms = resources.renderer.resources.sky.buffers.star_ubo.data_mut();
        star_uniforms.rotation = star_rotation;
        star_uniforms.tint = lighting_profile.stars_color;
        resources
            .renderer
            .resources
            .sky
            .buffers
            .star_ubo
            .write(queue);
    }
}

pub fn interpolate_weather_lighting_profiles(
    weather: WeatherComponent,
    time: f32,
) -> WeatherLightingProfile {
    // Each time range maps to a profile blend.
    let segments = [
        (
            MIDNIGHT,
            NIGHT,
            weather.midnight_lighting,
            weather.night_lighting,
        ),
        (NIGHT, DAWN, weather.night_lighting, weather.dawn_lighting),
        (
            DAWN,
            SUNRISE,
            weather.dawn_lighting,
            weather.sunrise_lighting,
        ),
        (
            SUNRISE,
            MORNING,
            weather.sunrise_lighting,
            weather.day_lighting,
        ),
        (MORNING, NOON, weather.day_lighting, weather.noon_lighting),
        (NOON, AFTERNOON, weather.noon_lighting, weather.day_lighting),
        (
            AFTERNOON,
            SUNSET,
            weather.day_lighting,
            weather.sunset_lighting,
        ),
        (SUNSET, DUSK, weather.sunset_lighting, weather.dusk_lighting),
        (DUSK, EVENING, weather.dusk_lighting, weather.night_lighting),
        (
            EVENING,
            MIDNIGHT_END,
            weather.night_lighting,
            weather.midnight_lighting,
        ),
    ];

    for &(start, end, a, b) in &segments {
        if time >= start && time < end {
            let t = (time - start) / (end - start);
            return lerp_lighting_profile(a, b, t);
        }
    }

    // Fallback in case of float rounding at exactly 1.0.
    weather.midnight_lighting
}

fn lerp_lighting_profile(
    a: WeatherLightingProfile,
    b: WeatherLightingProfile,
    t: f32,
) -> WeatherLightingProfile {
    WeatherLightingProfile {
        ambient_color: lerp(a.ambient_color, b.ambient_color, ease_in_out_quad(t)),
        ambient_strength: lerp(a.ambient_strength, b.ambient_strength, ease_in_quad(t)),
        fog_color: lerp(a.fog_color, b.fog_color, ease_in_out_quad(t)),
        fog_exponent: lerp(a.fog_exponent, b.fog_exponent, ease_in_quad(t)),
        fog_start: lerp(a.fog_start, b.fog_start, ease_out_quad(t)),
        fog_end: lerp(a.fog_end, b.fog_end, ease_out_quad(t)),
        sunlight_color: lerp(a.sunlight_color, b.sunlight_color, ease_in_out_quad(t)),
        sunlight_strength: lerp(a.sunlight_strength, b.sunlight_strength, ease_in_quad(t)),
        horizon_color: lerp(a.horizon_color, b.horizon_color, smoothstep(t)),
        sky_lower_color: lerp(a.sky_lower_color, b.sky_lower_color, ease_in_out_quad(t)),
        sky_upper_color: lerp(a.sky_upper_color, b.sky_upper_color, ease_in_out_quad(t)),
        cloud_layer_color: lerp(a.cloud_layer_color, b.cloud_layer_color, ease_out_quad(t)),
        stars_color: lerp(a.stars_color, b.stars_color, ease_in_out_cubic(t)),
        sun_color: lerp(a.sun_color, b.sun_color, ease_in_out_quad(t)),
        moon_color: lerp(a.moon_color, b.moon_color, ease_out_quad(t)),
        turbidity: lerp(a.turbidity, b.turbidity, ease_out_quad(t)),
    }
}

fn profile(
    ambient: [f32; 3],
    ambient_strength: f32,
    cloud: [f32; 3],
    fog: [f32; 3],
    fog_exponent: f32,
    fog_start: f32,
    fog_end: f32,
    horizon: [f32; 3],
    sky_lower: [f32; 3],
    sky_upper: [f32; 3],
    stars: [f32; 4],
    sun: [f32; 3],
    sunlight: [f32; 3],
    sunlight_strength: f32,
    moon: [f32; 4],
    turbidity: f32,
) -> WeatherLightingProfile {
    WeatherLightingProfile {
        ambient_color: Vec3::from(ambient),
        ambient_strength,
        cloud_layer_color: Vec3::from(cloud),
        fog_color: Vec3::from(fog),
        fog_exponent,
        fog_start,
        fog_end,
        horizon_color: Vec3::from(horizon),
        sky_lower_color: Vec3::from(sky_lower),
        sky_upper_color: Vec3::from(sky_upper),
        stars_color: Vec4::from(stars),
        sun_color: Vec3::from(sun),
        sunlight_color: Vec3::from(sunlight),
        sunlight_strength,
        moon_color: Vec4::from(moon),
        turbidity,
    }
}
