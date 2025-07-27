use crate::GameResources;
use crate::world::World;
use game_settings::{ACTIVE_CAMERA, DATA_PACKAGES};
use ron::ser::{PrettyConfig, to_string, to_string_pretty};
use sedona_settings::{SettingsValue, value_as};
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

pub fn save_game_package(world: &World, path: &str) {
    let string =
        to_string_pretty(world, PrettyConfig::default()).expect("RON serialization failed");

    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        create_dir_all(parent).expect("Failed to create parent directories for save path");
    }

    write(path_obj, string).expect("Failed to write RON file");
}

fn load_game_package(path: &Path) -> World {
    let file = std::fs::File::open(path).expect("Failed to open game package");
    let world: World = ron::de::from_reader(file).expect("Failed to deserialize game package");

    world
}

pub fn load_game_packages_into_world(world: &mut World, resources: &mut GameResources) {
    let Some(SettingsValue::Array(array)) = resources.config.get(DATA_PACKAGES) else {
        log::error!("Found no game packages to load");
        return;
    };

    for (index, value) in array.iter().enumerate() {
        match value {
            SettingsValue::String(file_name) => {
                let path = Path::new("data").join(file_name);
                let new_world = load_game_package(&path);
                world.merge(new_world);
            }
            other => {
                log::warn!(
                    "Expected string at index {} in {} but found {:?}",
                    index,
                    DATA_PACKAGES,
                    other
                );
            }
        }
    }
}
