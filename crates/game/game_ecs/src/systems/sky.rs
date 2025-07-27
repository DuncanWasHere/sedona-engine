use crate::GameResources;
use crate::utils::constants::WORLD_UP;
use game_settings::{
    GAME_TIME, LATITUDE, MOON_DIRECTION, MOON_PHASE, MOON_ROTATION, SOLAR_TIME, STAR_ROTATION,
    SUN_DIRECTION, SUN_ROTATION,
};
use glam::{Quat, Vec3};
use sedona_ecs::system;
use sedona_settings::{SettingsValue, value_as};
use std::f32::consts::{PI, TAU};

const AXIAL_TILT: f32 = 23.4397;

#[system(group=tick)]
pub fn sky_tick(resources: &mut GameResources) {
    let game_time = match resources.variables.get(GAME_TIME) {
        Some(SettingsValue::Time(value)) => *value,
        _ => return,
    };

    let day_of_year = game_time.day_of_year();
    let time_of_day = game_time.time_of_day_fraction();

    let latitude = value_as(resources.config.get(LATITUDE)).unwrap_or_default();

    // Sun
    let (sun_direction, sun_rotation, solar_time) =
        compute_sun_state(latitude, day_of_year, time_of_day);
    resources
        .variables
        .insert(SUN_DIRECTION, SettingsValue::Vec3(sun_direction));
    resources
        .variables
        .insert(SUN_ROTATION, SettingsValue::Quat(sun_rotation));
    resources
        .variables
        .insert(SOLAR_TIME, SettingsValue::Float(solar_time as f64));

    // Moon
    let (moon_direction, moon_rotation, moon_phase_index) =
        compute_moon_state(latitude, sun_direction, day_of_year, time_of_day);
    resources
        .variables
        .insert(MOON_DIRECTION, SettingsValue::Vec3(moon_direction));
    resources
        .variables
        .insert(MOON_ROTATION, SettingsValue::Quat(moon_rotation));
    resources
        .variables
        .insert(MOON_PHASE, SettingsValue::Integer(moon_phase_index as i64));

    let star_rotation = compute_star_state(latitude, day_of_year, time_of_day);
    resources
        .variables
        .insert(STAR_ROTATION, SettingsValue::Quat(star_rotation));
}

/// Compute the sun's position in the sky at a given local uniform time and latitude.
/// Also returns the solar time as the non-uniform local apparent time adjusted for latitude.
/// * `latitude` - Current latitude in degrees.
/// * `day_of_year` - Current day of year (1 - 365).
/// * `time_of_day` - Current local mean time of day (0.0 - 1.0).
fn compute_sun_state(latitude: f32, day_of_year: u32, time_of_day: f32) -> (Vec3, Quat, f32) {
    let latitude = latitude.to_radians();

    // Sun Declination Formula: δ = 23.45° * sin(360° / 365 * (n * 284))
    let sun_declination =
        AXIAL_TILT.to_radians() * (TAU / 365.0 * (day_of_year as f32 + 284.0)).sin();

    // Approximation for EoT
    let equation_of_time = {
        let b = (360.0 / 365.0) * (day_of_year as f32 - 81.0);
        let b_rad = b.to_radians();
        9.87 * (2.0 * b_rad).sin() - 7.53 * b_rad.cos() - 1.5 * b_rad.sin()
    };

    // Local Apparent Time is the mean time adjusted by the EoT
    let local_apparent_time = time_of_day * 24.0 + equation_of_time / 60.0;
    let solar_hour_angle = (15.0 * (local_apparent_time - 12.0)).to_radians();

    let altitude = (latitude.sin() * sun_declination.sin()
        + latitude.cos() * sun_declination.cos() * solar_hour_angle.cos())
    .asin();

    let sin_azimuth = -(solar_hour_angle.sin() * sun_declination.cos());
    let cos_azimuth = (sun_declination.sin() - latitude.sin() * altitude.sin())
        / (latitude.cos() * altitude.cos());

    let azimuth = sin_azimuth.atan2(cos_azimuth);

    // Sun direction in world space (Y-up)
    let direction = Vec3::new(
        altitude.cos() * azimuth.sin(),
        altitude.sin(),
        -altitude.cos() * azimuth.cos(),
    )
    .normalize();

    // Rotation quaternion to aim sun texture
    let base_direction = -WORLD_UP; // Sun texture faces down by default
    let rotation = Quat::from_rotation_arc(base_direction, direction);

    // Adjusted Solar Time (Local Apparent Time adjusted for latitude)
    // Not to be confused with Apparent Solar Time which is the same as Local Apparent Time.
    // Used for lighting calculations.
    // 0 = Midnight
    // 0.25 = Sunrise (Geometric)
    // 0.5 = Noon
    // 0.75 = Sunset (Geometric)

    // Compute geometric sunrise/sunset hour angle using the sunrise equation
    let cos_omega_0 = (-latitude.tan() * sun_declination.tan()).clamp(-1.0, 1.0);
    let omega_0 = cos_omega_0.acos(); // Angle of sunrise/sunset
    let is_daytime = solar_hour_angle.abs() <= omega_0;

    // Normalize solar hour angle into range [-PI, PI]
    let mut sha = solar_hour_angle;
    if sha < -PI {
        sha += TAU;
    } else if sha > PI {
        sha -= TAU;
    }

    let adjusted_solar_time = if is_daytime {
        // Daytime: interpolate 0.25 (sunrise) -> 0.75 (sunset)
        0.25 + ((sha + omega_0) / (2.0 * omega_0)) * 0.5
    } else if sha < -omega_0 {
        // After midnight -> before sunrise: 0.0 to 0.25
        let night_fraction = ((sha + TAU) - omega_0) / (PI - omega_0); // Wrap SHA forward by 2pi
        (night_fraction % 1.0) * 0.25
    } else {
        // After sunset -> before midnight: 0.75 to 1.0
        let night_fraction = (sha - omega_0) / (PI - omega_0);
        0.75 + night_fraction * 0.25
    };

    (direction, rotation, adjusted_solar_time)
}

/// Compute the moon's position in the sky at a given local uniform time and latitude.
/// Also returns the moon phase index calculated .
/// * `latitude` - Current latitude in degrees.
/// * `day_of_year` - Current day of year (1 - 365).
/// * `time_of_day` - Current local mean time of day (0.0 - 1.0).
fn compute_moon_state(
    latitude: f32,
    sun_direction: Vec3,
    day_of_year: u32,
    time_of_day: f32,
) -> (Vec3, Quat, u32) {
    const LUNAR_PERIOD: f32 = 29.53; // Days

    let latitude = latitude.to_radians();

    // Approximate Moon declination variation (+-5.1 deg tilt)
    let declination = 0.089 * (TAU * day_of_year as f32 / 365.25).sin(); // radians

    // Local lunar time
    let lunar_day = day_of_year as f32 % LUNAR_PERIOD;
    let moon_time = (time_of_day + lunar_day / LUNAR_PERIOD) % 1.0;
    let hour_angle = TAU * (moon_time - 0.5); // 0 = highest point

    let altitude = (latitude.sin() * declination.sin()
        + latitude.cos() * declination.cos() * hour_angle.cos())
    .asin();

    let sin_azimuth = -(hour_angle.sin() * declination.cos());
    let cos_azimuth =
        (declination.sin() - latitude.sin() * altitude.sin()) / (latitude.cos() * altitude.cos());

    let azimuth = sin_azimuth.atan2(cos_azimuth);

    // Moon direction in world space (Y-up)
    let direction = Vec3::new(
        altitude.cos() * azimuth.sin(),
        altitude.sin(),
        -altitude.cos() * azimuth.cos(),
    )
    .normalize();

    // Rotation quaternion to aim moon texture
    let base_direction = -WORLD_UP; // Moon texture faces down by default
    let rotation = Quat::from_rotation_arc(base_direction, direction);

    // Moon phase
    let phase = lunar_day / LUNAR_PERIOD;
    let phase_index = (phase * 8.0).floor() as u32 % 8;

    (direction, rotation, phase_index)
}

fn compute_star_state(latitude: f32, day_of_year: u32, time_of_day: f32) -> Quat {
    // Sidereal rotation over time: 360.9856° per solar day
    let sidereal_angle_deg = time_of_day * 360.9856 + (day_of_year as f32 * 0.9856);
    let sidereal_angle_rad = sidereal_angle_deg.to_radians();

    // Rotate sky clockwise (stars move east-to-west), Z axis = up in world space
    let sidereal_rotation = Quat::from_rotation_y(-sidereal_angle_rad);

    // Tilt celestial sphere based on latitude
    let latitude_tilt = Quat::from_rotation_x(latitude.to_radians());

    sidereal_rotation * latitude_tilt
}
