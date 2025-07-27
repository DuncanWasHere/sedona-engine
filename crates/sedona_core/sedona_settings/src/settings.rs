use crate::ENGINE_CONFIG_PATH;
use crate::settings_value::SettingsValue;
use ahash::{AHashMap, RandomState};
use sedona_utils::time::Time;
use std::fs;
use std::hash::{BuildHasher, Hasher};
use std::path::Path;
use toml::Value;

pub struct Settings {
    pub map: AHashMap<u64, SettingsValue>,
}

impl Settings {
    const FIXED_HASH_STATE: RandomState = RandomState::with_seeds(0, 0, 0, 0);

    pub fn new(key_count: usize) -> Self {
        let map = AHashMap::with_capacity_and_hasher(key_count, Self::FIXED_HASH_STATE);
        Self { map }
    }

    pub fn from_configs(game_config_paths: &[&str], game_config_key_count: usize) -> Self {
        let map = AHashMap::with_capacity_and_hasher(game_config_key_count, Self::FIXED_HASH_STATE);

        let mut settings = Self { map };

        settings.load_toml(ENGINE_CONFIG_PATH);

        for config_path in game_config_paths {
            settings.load_toml(config_path);
        }

        settings
    }

    fn load_toml(&mut self, path_str: &str) {
        let path = Path::new(path_str);
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(err) => {
                eprintln!("Failed to read config file at {}: {}", path.display(), err);
                return;
            }
        };

        let parsed: Value = match toml::from_str(&contents) {
            Ok(p) => p,
            Err(err) => {
                eprintln!("Failed to parse TOML config at {}: {}", path.display(), err);
                return;
            }
        };

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        if let Value::Table(table) = parsed {
            self.flatten_and_store(file_stem, &table);
        } else {
            eprintln!("Root of config file must be a table: {}", path.display());
        }
    }

    fn flatten_and_store(&mut self, prefix: &str, table: &toml::map::Map<String, Value>) {
        for (key, value) in table {
            let full_key = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            match value {
                Value::Table(subtable) => {
                    self.flatten_and_store(&full_key, subtable);
                }
                other => {
                    if let Some(global_value) = Self::toml_to_settings_value(other.clone()) {
                        let mut hasher = Self::FIXED_HASH_STATE.build_hasher();
                        hasher.write(full_key.as_bytes());
                        self.map.insert(hasher.finish(), global_value);
                    } else {
                        eprintln!("Unsupported or malformed config entry: {full_key}");
                    }
                }
            }
        }
    }

    fn toml_to_settings_value(value: Value) -> Option<SettingsValue> {
        match value {
            Value::String(s) => Some(SettingsValue::String(s)),
            Value::Boolean(b) => Some(SettingsValue::Bool(b)),
            Value::Integer(i) => Some(SettingsValue::Integer(i)),
            Value::Float(f) => Some(SettingsValue::Float(f)),
            Value::Datetime(dt) => {
                let time = Self::toml_datetime_to_settings_time(&dt)?;
                Some(SettingsValue::Time(time))
            }
            Value::Array(arr) => {
                let mut result = Vec::with_capacity(arr.len());
                for item in arr {
                    match Self::toml_to_settings_value(item) {
                        Some(val) => result.push(val),
                        None => return None, // Mixed type arrays are unsupported.
                    }
                }
                Some(SettingsValue::Array(result))
            }
            _ => None,
        }
    }

    fn toml_datetime_to_settings_time(dt: &toml::value::Datetime) -> Option<Time> {
        let date = dt.date?;
        let time = dt.time?;

        Some(Time::new(
            date.year as u32,
            date.month,
            date.day,
            time.hour,
            time.minute,
            time.second as f32 + (time.nanosecond as f32 / 1_000_000_000.0),
        ))
    }

    pub fn get(&self, key: u64) -> Option<&SettingsValue> {
        self.map.get(&key)
    }

    pub fn get_mut(&mut self, key: u64) -> Option<&mut SettingsValue> {
        self.map.get_mut(&key)
    }

    pub fn insert(&mut self, key: u64, value: SettingsValue) {
        self.map.insert(key, value);
    }
}
