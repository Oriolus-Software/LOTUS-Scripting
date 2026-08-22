//! Spielzeit-Darstellung, die zwischen Scripts und Engine geteilt wird.
//!
//! In-game time representation shared between scripts and the engine.

#[cfg(feature = "bevy")]
use bevy::{
    prelude::{ReflectResource, Resource},
    reflect::Reflect,
};

/// Aktuelle Simulationszeit basierend auf Unix-Mikrosekunden-Zeitstempeln.
///
/// Current simulation time based on Unix microsecond timestamps.
#[cfg_attr(feature = "bevy", derive(Resource, Reflect, Debug, Clone, Copy))]
#[cfg_attr(feature = "bevy", reflect(Resource))]
pub struct GameTime {
    #[cfg(feature = "engine")]
    speed_multiplier: f32,
    time_unix_micros: i64,
}

impl GameTime {
    /// Gibt die aktuelle Zeit als Unix-Mikrosekunden zurück.
    ///
    /// Returns the current time as Unix microseconds.
    pub fn time_unix_micros(&self) -> i64 {
        self.time_unix_micros
    }
}

#[cfg(feature = "engine")]
mod _engine {
    use time::PrimitiveDateTime;

    use super::*;

    const MICROS_IN_DAY: i64 = 86_400_000_000;

    impl GameTime {
        /// Erhöht die Simulationszeit um die angegebene Sekundenanzahl.
        ///
        /// Advances the simulation time by the given number of seconds.
        pub fn increase(&mut self, seconds: f32) {
            self.time_unix_micros += (seconds * 1_000_000.0 * self.speed_multiplier).round() as i64;
        }

        /// Gibt Tage seit der Frühlings-Tagundnachtgleiche 2024 zurück.
        ///
        /// Returns days since the vernal equinox of 2024.
        pub fn days_since_vernal_equinox_24(&self) -> f64 {
            let diff = self.time_unix_micros - 1_710_975_600_000_000;
            diff as f64 / 86_400_000_000.0
        }

        /// Setzt die Simulationszeit aus Datum und Uhrzeit.
        ///
        /// Sets the simulation time from a calendar date and time.
        pub fn set_time(&mut self, time: PrimitiveDateTime) {
            self.time_unix_micros = (time.assume_utc().unix_timestamp_nanos() / 1_000) as i64;
        }

        /// Gibt den aktuellen Simulationsgeschwindigkeitsfaktor zurück.
        ///
        /// Returns the current simulation speed multiplier.
        pub fn speed_multiplier(&self) -> f32 {
            self.speed_multiplier
        }

        /// Setzt den Simulationsgeschwindigkeitsfaktor.
        ///
        /// Sets the simulation speed multiplier.
        pub fn set_speed_multiplier(&mut self, speed_multiplier: f32) {
            self.speed_multiplier = speed_multiplier;
        }

        /// Gibt die Tageszeit normalisiert auf 0–1 zurück.
        ///
        /// Returns the time of day normalized to 0-1
        pub fn day_time(&self) -> f32 {
            (self.time_unix_micros % MICROS_IN_DAY) as f32 / MICROS_IN_DAY as f32
        }

        /// Gibt den Wochentag zurück; 0 = Montag.
        ///
        /// Returns the day of the week, 0 = Monday
        pub fn day_of_week(&self) -> u8 {
            ((self.time_unix_micros / MICROS_IN_DAY + 3) % 7) as u8
        }
    }
}

#[cfg(feature = "time")]
mod _time {
    use time::{Date, Duration, PrimitiveDateTime, Time};

    use super::*;

    impl GameTime {
        /// Wandelt die Simulationszeit in ein [`PrimitiveDateTime`] um.
        ///
        /// Converts the simulation time to a [`PrimitiveDateTime`].
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

impl GameTime {
    /// Erstellt eine Spielzeit aus Unix-Mikrosekunden.
    ///
    /// Creates a game time from Unix microseconds.
    pub fn from_unix_micros(unix_micros: i64) -> Self {
        Self {
            #[cfg(feature = "engine")]
            speed_multiplier: 1.0,
            time_unix_micros: unix_micros,
        }
    }
}

#[cfg(feature = "engine")]
impl Default for GameTime {
    fn default() -> Self {
        Self {
            speed_multiplier: 1.0,
            time_unix_micros: 1_746_957_625_000_000,
        }
    }
}
