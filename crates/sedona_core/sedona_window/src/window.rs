use sedona_io::load_rgba8_from_file;
use sedona_settings::*;
use std::sync::Arc;
use winit::dpi::{PhysicalSize, Size};
use winit::error::OsError;
use winit::event_loop::ActiveEventLoop;
use winit::window::{CursorGrabMode, Fullscreen, Icon, Window, WindowAttributes};

pub struct WindowContext {
    window: Arc<Window>,
    cursor_visible: bool,
}

impl WindowContext {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, OsError> {
        Self::with_attributes(WindowAttributes::default(), event_loop)
    }

    pub fn with_attributes(
        attributes: WindowAttributes,
        event_loop: &ActiveEventLoop,
    ) -> Result<Self, OsError> {
        let window = Arc::new(event_loop.create_window(attributes)?);

        Ok(Self {
            window,
            cursor_visible: true,
        })
    }

    pub fn from_config(config: &Settings, event_loop: &ActiveEventLoop) -> Result<Self, OsError> {
        let title = match config.get(WINDOW_TITLE) {
            Some(SettingsValue::String(value)) => value.clone(),
            _ => String::from("Sedona Engine"),
        };

        let window_icon = if let Some(SettingsValue::String(icon_path)) = config.get(WINDOW_ICON) {
            match load_rgba8_from_file(icon_path) {
                Ok((pixels, icon_width, icon_height)) => {
                    match Icon::from_rgba(pixels, icon_width, icon_height) {
                        Ok(icon) => Some(icon),
                        Err(err) => {
                            eprintln!("Failed to create icon from RGBA8 data: {}", err);
                            None
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to load icon image '{}': {}", icon_path, err);
                    None
                }
            }
        } else {
            None
        };

        let fullscreen: bool = value_as(config.get(FULLSCREEN)).unwrap_or_default();
        let width: Option<u32> = value_as(config.get(WINDOW_WIDTH));
        let height: Option<u32> = value_as(config.get(WINDOW_HEIGHT));

        let mut attributes = WindowAttributes::default();

        attributes.title = title;
        attributes.window_icon = window_icon;
        attributes.fullscreen = if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };
        attributes.inner_size = if !fullscreen {
            if let (Some(width), Some(height)) = (width, height) {
                Some(Size::from(PhysicalSize::new(width, height)))
            } else {
                None
            }
        } else {
            None
        };

        Self::with_attributes(attributes, event_loop)
    }

    pub fn set_fullscreen(&self) {
        self.window
            .set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    pub fn set_windowed(&self) {
        self.window.set_fullscreen(None);
    }

    pub fn lock_cursor(&self) {
        match self.window.set_cursor_grab(CursorGrabMode::Locked) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to lock cursor: {}", err);
            }
        }
    }

    pub fn unlock_cursor(&self) {
        match self.window.set_cursor_grab(CursorGrabMode::None) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Failed to unlock cursor: {}", err);
            }
        }
    }

    pub fn hide_cursor(&mut self) {
        self.window.set_cursor_visible(false);
        self.cursor_visible = false;
    }

    pub fn show_cursor(&mut self) {
        self.window.set_cursor_visible(true);
        self.cursor_visible = true;
    }

    pub fn pre_present_notify(&self) {
        self.window.pre_present_notify()
    }

    pub fn window(&self) -> &Arc<Window> {
        &self.window
    }

    pub fn fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    pub fn cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn size(&self) -> (u32, u32) {
        self.window.inner_size().into()
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw()
    }
}
