#[cfg(feature = "bevy")]
use bevy::{prelude::Resource, reflect::Reflect};
use time::PrimitiveDateTime;

#[cfg(feature = "bevy")]
#[derive(Resource, Reflect)]
pub struct GameTime {
    time_unix_micros: i64,
}

#[cfg(not(feature = "bevy"))]
pub struct GameTime {
    time_unix_micros: i64,
}

impl GameTime {
    pub fn increase(&mut self, seconds: f32) {
        self.time_unix_micros += (seconds * 1_000_000.0).round() as i64;
    }

    pub fn days_since_vernal_equinox_24(&self) -> f64 {
        (self.time_unix_micros - 1710975600000000) as f64 / 86400.0
    }

    pub fn set_time(&mut self, time: PrimitiveDateTime) {
        self.time_unix_micros = (time.assume_utc().unix_timestamp_nanos() / 1_000) as i64;
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            // 2024-06-21 12:00:00
            time_unix_micros: 1_718_928_000_000_000,
        }
    }
}
