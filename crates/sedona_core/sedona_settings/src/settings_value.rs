use glam::{Quat, Vec2, Vec3, Vec4};
use sedona_utils::time::Time;
use std::collections::HashMap;
use uuid::Uuid;

pub type Array = Vec<SettingsValue>;
pub type Table = HashMap<String, SettingsValue>;

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsValue {
    Array(Array),
    Bool(bool),
    EntityId(Uuid),
    Float(f64),
    Integer(i64),
    Quat(Quat),
    String(String),
    Table(Table),
    Time(Time),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
}

impl SettingsValue {
    pub fn as_float_lossy(&self) -> Option<f64> {
        match self {
            SettingsValue::Float(f) => Some(*f),
            SettingsValue::Integer(i) => Some(*i as f64),
            SettingsValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    pub fn as_int_lossy(&self) -> Option<i64> {
        match self {
            SettingsValue::Integer(i) => Some(*i),
            SettingsValue::Float(f) => Some(*f as i64),
            SettingsValue::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    pub fn as_bool_lossy(&self) -> Option<bool> {
        match self {
            SettingsValue::Bool(b) => Some(*b),
            SettingsValue::Integer(i) => Some(*i != 0),
            SettingsValue::Float(f) => Some(*f != 0.0),
            _ => None,
        }
    }
}

pub trait FromSettingsValue: Sized {
    fn from_settings_value(value: &SettingsValue) -> Option<Self>;
}

pub fn value_as<T: FromSettingsValue>(opt: Option<&SettingsValue>) -> Option<T> {
    opt.and_then(|v| T::from_settings_value(v))
}

impl FromSettingsValue for i64 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_int_lossy()
    }
}

impl FromSettingsValue for i32 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_int_lossy().map(|v| v as i32)
    }
}

impl FromSettingsValue for u64 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_int_lossy().map(|v| v as u64)
    }
}

impl FromSettingsValue for u32 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_int_lossy().map(|v| v as u32)
    }
}

impl FromSettingsValue for usize {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_int_lossy().map(|v| v as usize)
    }
}

impl FromSettingsValue for f64 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_float_lossy()
    }
}

impl FromSettingsValue for f32 {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_float_lossy().map(|v| v as f32)
    }
}

impl FromSettingsValue for bool {
    fn from_settings_value(value: &SettingsValue) -> Option<Self> {
        value.as_bool_lossy()
    }
}
