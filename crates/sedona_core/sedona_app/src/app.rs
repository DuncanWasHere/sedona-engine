use crate::Game;
use winit::{application::*, event::*, event_loop::*, keyboard::*, window::*};

pub struct App<G: Game> {
    game: G,
}

impl<G: Game> App<G> {
    pub fn new(game: G) -> Self {
        Self { game: game }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self).expect("Application exited");
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Running game pre-startup stage...");
        self.game.pre_startup();

        log::info!("Initializing resources...");
        self.game.initialize_resources(event_loop);

        log::info!("Running game startup stage...");
        self.game.startup();

        log::info!("Running game post-startup stage...");
        self.game.post_startup();
    }
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Winit recommends instancing the window after the first resume input.
        // This input should only fire once on desktops but who knows.
        if !self.game.resources_initialized() {
            self.initialize(event_loop);
            return;
        }

        log::debug!("Resuming game...");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Can't assume this input won't fire before resumed? Unsure.
        if !self.game.resources_initialized() {
            log::warn!("Window event was fired but window is not initialized");
            return;
        };

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Window was requested to close. Stopping...");
                self.game.quit();

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.game.first();

                self.game.update();

                self.game.pre_update();
                self.game.update();
                self.game.post_update();

                self.game.render();

                self.game.last();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let key = event.physical_key;
                let keycode = if let PhysicalKey::Code(k) = key {
                    Some(k)
                } else {
                    None
                };

                if let Some(keycode) = keycode {
                    match event.state {
                        ElementState::Pressed => {
                            self.game.key_down(keycode);
                        }
                        ElementState::Released => {
                            self.game.key_up(keycode);
                        }
                    }
                }
            }
            WindowEvent::Resized(size) => {
                self.game.window_resized(size);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.game.mouse_wheel(delta);
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.game.mouse_motion(delta);
            }
            _ => {
                self.game.device_event(&event);
            }
        }
    }
}
