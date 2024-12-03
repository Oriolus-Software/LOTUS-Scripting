#[cfg(feature = "bevy")]
use bevy::{
    prelude::{ReflectResource, Resource},
    reflect::Reflect,
};
use time::PrimitiveDateTime;

#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Resource))]
pub struct GameTime {
    time_unix_micros: i64,
}

impl GameTime {
    pub fn increase(&mut self, seconds: f32) {
        self.time_unix_micros += (seconds * 1_000_000.0).round() as i64;
    }

    pub fn days_since_vernal_equinox_24(&self) -> f64 {
        let diff = self.time_unix_micros - 1_710_975_600_000_000;
        let r = diff as f64 / 86_400_000_000.0;
        r
    }

    pub fn set_time(&mut self, time: PrimitiveDateTime) {
        self.time_unix_micros = (time.assume_utc().unix_timestamp_nanos() / 1_000) as i64;
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            // 2024-06-21 12:00:00
            time_unix_micros: 1718967600000000,
        }
    }
}
