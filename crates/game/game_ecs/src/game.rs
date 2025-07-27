use crate::events::KeyEvent;
use crate::world::{
    World, systems_first, systems_last, systems_mouse_wheel, systems_post_startup,
    systems_post_tick, systems_post_update, systems_pre_startup, systems_pre_tick,
    systems_pre_update, systems_quit, systems_startup, systems_tick, systems_tick_first,
    systems_tick_last, systems_update, systems_window_resized,
};
use crate::{GameEventHandlers, GameEventQueues};
use game_settings::{DATA_PACKAGES, GAME_CONFIG_KEY_COUNT, GAME_CONFIG_PATH, GAME_VARIABLE_KEY_COUNT, TIME_SCALE};
use sedona_app::{ActiveEventLoop, DeviceEvent, Game, KeyCode, MouseScrollDelta, PhysicalSize};
use sedona_audio::AudioState;
use sedona_io::InputState;
use sedona_renderer::renderer::Renderer;
use sedona_resource::resources::Resources;
use sedona_resource::ticker::Ticker;
use sedona_settings::{Settings, value_as, SettingsValue};
use sedona_window::window::WindowContext;
use crate::utils::serialize::load_game_packages_into_world;

pub type GameResources = Resources<GameEventQueues>;

#[derive(Default)]
pub struct BigBerg {
    pub world: World,
    pub resources: Option<GameResources>,
    pub event_handlers: GameEventHandlers,
}

impl Game for BigBerg {
    fn initialize_resources(&mut self, event_loop: &ActiveEventLoop) {
        let variables = Settings::new(GAME_VARIABLE_KEY_COUNT);
        let config = Settings::from_configs(&[GAME_CONFIG_PATH], GAME_CONFIG_KEY_COUNT);

        let mut window = match WindowContext::from_config(&config, event_loop) {
            Ok(window) => window,
            Err(error) => panic!("Failed to create window: {error}"),
        };
        window.lock_cursor();
        window.hide_cursor();

        let renderer = Renderer::new(&window, &config);
        let audio_state = AudioState::from_config(&config);
        let ticker = Ticker::from_config(&config);
        let input_state = InputState::new();
        let event_queues = GameEventQueues::default();

        let resources = Resources {
            config,
            variables,
            window,
            renderer,
            audio_state,
            input_state,
            ticker,
            event_queues,
        };

        self.resources = Some(resources);
    }

    fn pre_startup(&mut self) {
        systems_pre_startup(&mut self.event_handlers, &mut self.world);
    }

    fn startup(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        load_game_packages_into_world(&mut self.world, resources);

        systems_startup(&mut self.event_handlers, resources, &mut self.world);
        resources.ticker.start();
    }

    fn post_startup(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_post_startup(&mut self.event_handlers, resources, &mut self.world);
    }

    fn first(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        let dt = resources.input_state.dt();

        let time_scale = value_as(resources.variables.get(TIME_SCALE)).unwrap_or(1.0);

        let tick_info = resources.ticker.update(dt, time_scale);

        for _ in 0..tick_info.num_ticks {
            self.tick_first();
            self.pre_tick();
            self.tick();
            self.post_tick();
            self.tick_last();
        }

        let resources = self.resources.as_mut().unwrap();
        systems_first(resources, &mut self.world);
    }

    fn pre_update(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_pre_update(resources, &mut self.world);
    }

    fn update(&mut self) {
        let resources = self.resources.as_mut().unwrap();
        self.event_handlers.dispatch_all(&mut self.world, resources);

        systems_update(resources, &mut self.world);
    }

    fn post_update(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_post_update(resources, &mut self.world);
    }

    fn last(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_last(resources, &mut self.world);
    }

    fn render(&mut self) {
        if let Some(resources) = self.resources.as_mut() {
            resources.input_state.reset_frame_time();
            resources.input_state.reset_frame_input();
            resources.renderer.render(&resources.window);
            resources.window.request_redraw();
        }
    }

    fn quit(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_quit(resources, &mut self.world);
    }

    fn tick_first(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_tick_first(resources, &mut self.world);
    }

    fn pre_tick(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_pre_tick(resources, &mut self.world);
    }

    fn tick(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_tick(resources, &mut self.world);
    }

    fn post_tick(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_post_tick(resources, &mut self.world);
    }

    fn tick_last(&mut self) {
        let resources = self.resources.as_mut().unwrap();

        systems_tick_last(resources, &mut self.world);
    }

    fn key_down(&mut self, key: KeyCode) {
        let resources = self.resources.as_mut().unwrap();
        resources.input_state.keys_held.insert(key);
        resources.event_queues.key_down.push_event(KeyEvent { key })
    }

    fn key_up(&mut self, key: KeyCode) {
        let resources = self.resources.as_mut().unwrap();
        resources.input_state.keys_held.remove(&key);
        resources.event_queues.key_up.push_event(KeyEvent { key })
    }

    fn window_resized(&mut self, _size: PhysicalSize<u32>) {
        let resources = self.resources.as_mut().unwrap();

        systems_window_resized(resources, &mut self.world);
    }

    fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
        let resources = self.resources.as_mut().unwrap();

        let y = match delta {
            MouseScrollDelta::LineDelta(_, y) => y,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 50.0,
        };

        systems_mouse_wheel(resources, &mut self.world, y);
    }

    fn mouse_motion(&mut self, delta: (f64, f64)) {
        let resources = self.resources.as_mut().unwrap();

        resources.input_state.add_mouse_delta(delta.0, delta.1);
    }

    fn device_event(&mut self, event: &DeviceEvent) {
        let resources = self.resources.as_mut().unwrap();
    }

    fn resources_initialized(&self) -> bool {
        self.resources.is_some()
    }
}
