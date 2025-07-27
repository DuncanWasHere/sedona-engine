use kira::sound::static_sound::StaticSoundHandle;
use kira::{AudioManager, AudioManagerSettings};
use sedona_settings::Settings;
use std::collections::HashMap;

pub struct AudioState {
    pub manager: AudioManager,
    pub sounds: HashMap<u64, StaticSoundHandle>,
}

impl AudioState {
    pub fn new() -> Self {
        let settings = AudioManagerSettings::default();
        let manager = match AudioManager::new(settings) {
            Ok(audio_manager) => audio_manager,
            Err(error) => panic!("Couldn't create audio manager: {}", error),
        };

        Self {
            manager,
            sounds: HashMap::new(),
        }
    }

    pub fn from_config(config: &Settings) -> Self {
        //TODO: Implement audio settings.

        let settings = AudioManagerSettings::default();
        let manager = match AudioManager::new(settings) {
            Ok(audio_manager) => audio_manager,
            Err(error) => panic!("Couldn't create audio manager: {}", error),
        };

        Self {
            manager,
            sounds: HashMap::new(),
        }
    }
}
