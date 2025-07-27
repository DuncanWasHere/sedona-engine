use glam::Vec3;

pub const WORLD_UP: Vec3 = Vec3::Y;
pub const WORLD_DOWN: Vec3 = Vec3::NEG_Y;
pub const NORTH: Vec3 = Vec3::Z;
pub const EAST: Vec3 = Vec3::X;
pub const SOUTH: Vec3 = Vec3::NEG_Z;
pub const WEST: Vec3 = Vec3::NEG_X;

pub const DEFAULT_TIME_SCALE: f32 = 1.0;
pub const DEFAULT_DAY_SCALE: f32 = 70.0; // 20 Minutes
pub const DEFAULT_START_YEAR: u32 = 2025;
pub const DEFAULT_START_MONTH: u8 = 6;
pub const DEFAULT_START_DAY: u8 = 1;
pub const DEFAULT_START_HOUR: u8 = 6;
pub const DEFAULT_START_MINUTE: u8 = 0;
pub const DEFAULT_START_SECOND: f32 = 0.0;
pub const MIDNIGHT: f32 = 0.0;
pub const NIGHT: f32 = 0.125;
pub const DAWN: f32 = 0.2225;
pub const SUNRISE: f32 = 0.26;
pub const MORNING: f32 = 0.3625;
pub const NOON: f32 = 0.50;
pub const AFTERNOON: f32 = 0.6425;
pub const SUNSET: f32 = 0.745;
pub const DUSK: f32 = 0.7775;
pub const EVENING: f32 = 0.875;
pub const MIDNIGHT_END: f32 = 1.00;
