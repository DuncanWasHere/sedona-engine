use crate::ticker::Ticker;
use sedona_audio::AudioState;
use sedona_io::InputState;
use sedona_renderer::renderer::Renderer;
use sedona_settings::Settings;
use sedona_window::window::WindowContext;

pub struct Resources<T> {
    pub config: Settings,
    pub variables: Settings,
    pub window: WindowContext,
    pub renderer: Renderer,
    pub audio_state: AudioState,
    pub input_state: InputState,
    pub ticker: Ticker,
    pub event_queues: T,
}
