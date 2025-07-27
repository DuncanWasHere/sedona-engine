use sedona_settings::{
    BASE_TICK_RATE_SCALE, MAX_TICKS_PER_FRAME, Settings, SettingsValue, value_as,
};

pub struct Ticker {
    accumulator: f32,
    tick_duration: f32,
    max_ticks_per_frame: usize,
    status: TickerStatus,
}

#[derive(Debug, Clone, Copy)]
pub struct TickInfo {
    pub num_ticks: usize,
    pub alpha: f32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TickerStatus {
    NotStarted,
    Running,
    Paused,
}

impl Ticker {
    pub const DEFAULT_TICK_DURATION: f32 = 1.0;
    pub const DEFAULT_MAX_TICKS_PER_FRAME: usize = 5;

    pub fn new(tick_duration: f32, max_ticks_per_frame: usize) -> Self {
        Self {
            accumulator: 0.0,
            tick_duration,
            max_ticks_per_frame,
            status: TickerStatus::NotStarted,
        }
    }

    pub fn from_config(config: &Settings) -> Self {
        let tick_duration = value_as(config.get(BASE_TICK_RATE_SCALE))
            .unwrap_or(Self::DEFAULT_TICK_DURATION)
            / 60.0;

        let max_ticks_per_frame =
            value_as(config.get(MAX_TICKS_PER_FRAME)).unwrap_or(Self::DEFAULT_MAX_TICKS_PER_FRAME);

        Self::new(tick_duration, max_ticks_per_frame)
    }

    pub fn update(&mut self, frame_dt: f32, time_scale: f32) -> TickInfo {
        if self.status == TickerStatus::Running {
            let time_scale = time_scale.max(0.0);
            let scaled_dt = frame_dt * time_scale;

            self.accumulator += scaled_dt;

            // Cap how far behind the simulation can fall.
            let max_accumulator = self.tick_duration * self.max_ticks_per_frame as f32 * 2.0;
            self.accumulator = self.accumulator.min(max_accumulator);

            let available_ticks = (self.accumulator / self.tick_duration).floor() as usize;
            let num_ticks = available_ticks.min(self.max_ticks_per_frame);

            if available_ticks > self.max_ticks_per_frame {
                log::debug!(
                    "Ticker: dropped {} ticks due to frame cap",
                    available_ticks - self.max_ticks_per_frame
                );
            }

            self.accumulator -= num_ticks as f32 * self.tick_duration;

            // Compute interpolation factor for render smoothing
            let raw_alpha = self.accumulator / self.tick_duration;
            let alpha = raw_alpha.clamp(0.0, 1.0);

            TickInfo { num_ticks, alpha }
        } else {
            TickInfo {
                num_ticks: 0,
                alpha: self.accumulator / self.tick_duration,
            }
        }
    }

    pub fn start(&mut self) {
        self.status = TickerStatus::Running;
    }

    pub fn pause(&mut self) {
        self.status = TickerStatus::Paused;
    }

    pub fn status(&self) -> TickerStatus {
        self.status
    }
}

impl Default for Ticker {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_TICK_DURATION,
            Self::DEFAULT_MAX_TICKS_PER_FRAME,
        )
    }
}
