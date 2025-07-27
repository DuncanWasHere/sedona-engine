use sedona_app::KeyCode;
use sedona_event::Event;

pub struct KeyEvent {
    pub key: KeyCode,
}

impl Event for KeyEvent {
    type Key = KeyCode;

    fn key(&self) -> Self::Key {
        self.key
    }
}
