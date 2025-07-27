use sedona_settings::define_global_keys;

pub const GAME_CONFIG_PATH: &str = "config/game.toml";

define_global_keys! {
    // DATA
    DATA_PACKAGES => "game.data.packages",

    // GRAPHICS
    FOV => "game.graphics.fov",

    // CONTROLS
    MOUSE_SENSITIVITY => "game.controls.mouse_sensitivity",

    // TIME
    DAY_SCALE => "game.time.day_scale",
    START_TIME => "game.time.start_time",
    LATITUDE => "game.time.latitude",
}
