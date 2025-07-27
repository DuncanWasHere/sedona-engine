use sedona_settings_macros::define_global_keys;

pub const ENGINE_CONFIG_PATH: &str = "config/engine.toml";

define_global_keys! {
    // GENERAL
    BASE_TICK_RATE_SCALE => "engine.general.base_tick_rate_scale",
    MAX_TICKS_PER_FRAME => "engine.general.max_ticks_per_frame",

    // RESOLUTION
    AUTO_RESOLUTION => "engine.resolution.auto_resolution",
    RESOLUTION_WIDTH => "engine.resolution.width",
    RESOLUTION_HEIGHT => "engine.resolution.height",

    // WINDOW
    WINDOW_ICON => "engine.window.icon",
    WINDOW_TITLE => "engine.window.title",
    WINDOW_WIDTH => "engine.window.width",
    WINDOW_HEIGHT => "engine.window.height",
    FULLSCREEN => "engine.window.fullscreen",

    // RENDERER
    INITIAL_RENDER_OBJECT_ARRAY_CAPACITY => "engine.renderer.initial_render_object_capacity",

    // TEXTURES
    FILTER_TEXTURES => "engine.textures.filter_textures",

    // SHADOW MAPS
    SHADOW_MAP_CASCADE_COUNT => "engine.shadow_map.cascade_count",
    SHADOW_MAP_RESOLUTION => "engine.shadow_map.resolution",

    // SKY
    SKY_PBR => "engine.sky.use_pbr",

    // SHADER PATHS
    ADAPTATION_SHADER_PATH => "engine.adaptation_shader",
    BLIT_SHADER_PATH => "engine.shader_paths.blit",
    BLIT_DEPTH_SHADER_PATH => "engine.shader_paths.blit_depth",
    BLOOM_COMPOSITE_SHADER_PATH => "engine.shader_paths.bloom_composite",
    BLOOM_EXTRACT_SHADER_PATH => "engine.shader_paths.bloom_extract",
    BLUR_SHADER_PATH => "engine.shader_paths.blur",
    CLOUD_SHADER_PATH => "engine.shader_paths.cloud",
    DEPTH_OF_FIELD_SHADER_PATH => "engine.shader_paths.depth_of_field",
    LENS_FLARE_SHADER_PATH => "engine.shader_paths.lens_flare",
    LUMINANCE_DOWNSAMPLE_SHADER_PATH => "engine.shader_paths.luminance_downsample",
    MOON_SHADER_PATH => "engine.shader_paths.moon",
    PBR_SHADER_PATH => "engine.shader_paths.pbr",
    PRE_PASS_SHADER_PATH => "engine.shader_paths.pre_pass",
    SCREEN_SPACE_AO_SHADER_PATH => "engine.shader_paths.screen_space_ao",
    SCREEN_SPACE_REFLECTION_SHADER_PATH => "engine.shader_paths.screen_space_reflection",
    SCREEN_SPACE_SHADOW_SHADER_PATH => "engine.shader_paths.screen_space_shadow",
    SHADOW_SHADER_PATH => "engine.shader_paths.shadow",
    SKY_BOX_SHADER_PATH => "engine.shader_paths.sky_box",
    SKY_GRADIENT_SHADER_PATH => "engine.shader_paths.sky_gradient",
    SKY_PBR_SHADER_PATH => "engine.shader_paths.sky_pbr",
    STAR_MAP_SHADER_PATH => "engine.shader_paths.star_map",
    SUN_SHADER_PATH => "engine.shader_paths.sun",
    TONE_MAP_SHADER_PATH => "engine.shader_paths.tone_map",
    UNLIT_SHADER_PATH => "engine.shader_paths.unlit",
}
