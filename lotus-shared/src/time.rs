#[cfg(feature = "bevy")]
use bevy::{
    prelude::{ReflectResource, Resource},
    reflect::Reflect,
};

#[cfg_attr(feature = "bevy", derive(Resource, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Resource))]
pub struct GameTime {
    #[cfg(feature = "engine")]
    speed_multiplier: f32,
    time_unix_micros: i64,
}

impl GameTime {
    pub fn time_unix_micros(&self) -> i64 {
        self.time_unix_micros
    }
}

#[cfg(feature = "engine")]
mod _engine {
    use time::PrimitiveDateTime;

    use super::*;

    impl GameTime {
        pub fn increase(&mut self, seconds: f32) {
            self.time_unix_micros += (seconds * 1_000_000.0 * self.speed_multiplier).round() as i64;
        }

        pub fn days_since_vernal_equinox_24(&self) -> f64 {
            let diff = self.time_unix_micros - 1_710_975_600_000_000;
            diff as f64 / 86_400_000_000.0
        }

        pub fn set_time(&mut self, time: PrimitiveDateTime) {
            self.time_unix_micros = (time.assume_utc().unix_timestamp_nanos() / 1_000) as i64;
        }

        pub fn speed_multiplier(&self) -> f32 {
            self.speed_multiplier
        }

        pub fn set_speed_multiplier(&mut self, speed_multiplier: f32) {
            self.speed_multiplier = speed_multiplier;
        }
    }
}

#[cfg(feature = "time")]
mod _time {
    use time::{Date, Duration, PrimitiveDateTime, Time};

    use super::*;

    impl GameTime {
        pub fn primitive_date_time(&self) -> PrimitiveDateTime {
            PrimitiveDateTime::new(
                Date::from_calendar_date(1970, time::Month::January, 1).unwrap(),
                Time::from_hms(0, 0, 0).unwrap(),
            ) + Duration::new(
                self.time_unix_micros / 1_000_000,
                (self.time_unix_micros % 1_000_000 * 1_000) as i32,
            )
        }
    }
}

#[cfg(feature = "engine")]
impl Default for GameTime {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            // 2024-06-21 12:00:00
            time_unix_micros: 1718967600000000,
        }
    }
}
