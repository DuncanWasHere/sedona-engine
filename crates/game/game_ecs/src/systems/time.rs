use crate::events::KeyEvent;
use crate::utils::constants::*;
use crate::world::World;
use crate::{GameEventHandlers, GameResources};
use game_settings::{
    DAY_SCALE, GAME_TIME, MOON_PHASE, SOLAR_TIME, START_TIME, SUN_DIRECTION, SUN_ROTATION,
    TIME_SCALE,
};
use sedona_app::KeyCode;
use sedona_ecs::system;
use sedona_settings::{BASE_TICK_RATE_SCALE, SettingsValue, value_as};
use sedona_utils::time::Time;

#[system(group=startup)]
pub fn time_startup(resources: &mut GameResources, event_handlers: &mut GameEventHandlers) {
    event_handlers.key_down.register(KeyCode::KeyP, print_time);
    event_handlers.key_down.register(KeyCode::KeyQ, pause_time);
    event_handlers
        .key_down
        .register(KeyCode::KeyE, fast_forward_time_enable);
    event_handlers
        .key_up
        .register(KeyCode::KeyE, fast_forward_time_disable);

    resources
        .variables
        .insert(TIME_SCALE, SettingsValue::Float(1.0));

    let game_time = match resources.config.get(START_TIME) {
        Some(SettingsValue::Time(value)) => *value,
        _ => Time::new(
            DEFAULT_START_YEAR,
            DEFAULT_START_MONTH,
            DEFAULT_START_DAY,
            DEFAULT_START_HOUR,
            DEFAULT_START_MINUTE,
            DEFAULT_START_SECOND,
        ),
    };

    resources
        .variables
        .insert(GAME_TIME, SettingsValue::Time(game_time));
}

#[system(group=tick)]
pub fn time_tick(resources: &mut GameResources) {
    let tick_duration = value_as(resources.config.get(BASE_TICK_RATE_SCALE)).unwrap_or(1.0) / 60.0;

    let day_scale = value_as(resources.config.get(DAY_SCALE)).unwrap_or(DEFAULT_DAY_SCALE);

    let current_time = match resources.variables.get_mut(GAME_TIME) {
        Some(SettingsValue::Time(value)) => value,
        None => panic!("Time System: Global variable game_time not found"),
        _ => panic!("Time System: Global variable game_time is the wrong type"),
    };

    current_time.increment_seconds_f32(tick_duration * day_scale);
}

fn print_time(_event: &KeyEvent, _world: &mut World, resources: &mut GameResources) {
    let game_time = match resources.variables.get(GAME_TIME) {
        Some(SettingsValue::Time(value)) => *value,
        _ => return,
    };
    let solar_time = match value_as(resources.variables.get(SOLAR_TIME)) {
        Some(time) => time,
        None => return,
    };
    let sun_direction = match resources.variables.get(SUN_DIRECTION) {
        Some(SettingsValue::Vec3(value)) => *value,
        _ => return,
    };
    let sun_rotation = match resources.variables.get(SUN_ROTATION) {
        Some(SettingsValue::Quat(value)) => *value,
        _ => return,
    };
    let moon_phase: f32 = match value_as(resources.variables.get(MOON_PHASE)) {
        Some(phase) => phase,
        None => return,
    };

    println!("Game time: {game_time}");
    println!("Solar time: {solar_time}");
    println!("Sun direction: {sun_direction}");
    println!("Sun rotation: {sun_rotation}");
    println!("Moon phase: {moon_phase}");
    print_time_interpolation_range(solar_time);
}

fn pause_time(_event: &KeyEvent, _world: &mut World, resources: &mut GameResources) {
    let time_scale = match resources.variables.get_mut(TIME_SCALE) {
        Some(SettingsValue::Float(value)) => value,
        None => panic!("TimeSystem: Global variable time_scale not found"),
        _ => panic!("TimeSystem: Global variable time_scale is the wrong type"),
    };

    if *time_scale == 0.0 {
        *time_scale = 1.0;
    } else {
        *time_scale = 0.0;
    }
}

fn fast_forward_time_enable(_event: &KeyEvent, _world: &mut World, resources: &mut GameResources) {
    let time_scale = match resources.variables.get_mut(TIME_SCALE) {
        Some(SettingsValue::Float(value)) => value,
        None => panic!("TimeSystem: Global variable time_scale not found"),
        _ => panic!("TimeSystem: Global variable time_scale is the wrong type"),
    };

    *time_scale = 15.0;
}

fn fast_forward_time_disable(_event: &KeyEvent, _world: &mut World, resources: &mut GameResources) {
    let time_scale = match resources.variables.get_mut(TIME_SCALE) {
        Some(SettingsValue::Float(value)) => value,
        None => panic!("TimeSystem: Global variable time_scale not found"),
        _ => panic!("TimeSystem: Global variable time_scale is the wrong type"),
    };

    *time_scale = 1.0;
}

fn print_time_interpolation_range(time: f32) {
    let segments = [
        (MIDNIGHT, NIGHT, "midnight", "night"),
        (NIGHT, DAWN, "night", "dawn"),
        (DAWN, SUNRISE, "dawn", "sunrise"),
        (SUNRISE, MORNING, "sunrise", "morning"),
        (MORNING, NOON, "morning", "noon"),
        (NOON, AFTERNOON, "noon", "afternoon"),
        (AFTERNOON, SUNSET, "afternoon", "sunset"),
        (SUNSET, DUSK, "sunset", "dusk"),
        (DUSK, EVENING, "dusk", "evening"),
        (EVENING, MIDNIGHT_END, "evening", "midnight"),
    ];

    for &(start, end, label_a, label_b) in &segments {
        if time >= start && time < end {
            println!(
                "Time {:.3} is between {:.3} ({}) and {:.3} ({})",
                time, start, label_a, end, label_b
            );
            return;
        }
    }

    if (time - 1.0).abs() < f32::EPSILON {
        println!(
            "Time {:.3} is exactly 1.0, using fallback to midnight",
            time
        );
    } else {
        println!("Time {:.3} is out of expected range (0.0 to 1.0)", time);
    }
}
