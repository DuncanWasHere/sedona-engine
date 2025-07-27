use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

#[allow(unused_variables)]
pub trait Game {
    fn initialize_resources(&mut self, event_loop: &ActiveEventLoop) {}

    fn render(&mut self) {}

    fn pre_startup(&mut self) {}

    fn startup(&mut self) {}

    fn post_startup(&mut self) {}

    fn first(&mut self) {}

    fn pre_update(&mut self) {}

    fn update(&mut self) {}

    fn post_update(&mut self) {}

    fn last(&mut self) {}

    fn quit(&mut self) {}

    fn tick_first(&mut self) {}

    fn pre_tick(&mut self) {}

    fn tick(&mut self) {}

    fn post_tick(&mut self) {}

    fn tick_last(&mut self) {}

    fn key_down(&mut self, key: KeyCode) {}

    fn key_up(&mut self, key: KeyCode) {}

    fn window_resized(&mut self, size: PhysicalSize<u32>) {}

    fn mouse_motion(&mut self, delta: (f64, f64)) {}

    fn mouse_wheel(&mut self, delta: MouseScrollDelta) {}

    fn device_event(&mut self, event: &DeviceEvent) {}

    fn resources_initialized(&self) -> bool;
}
