//! Simulationszeit und Tick-Zähler.
//!
//! Simulation time and tick counters.

/// Gibt die vergangene Zeit seit dem letzten Tick in Sekunden zurück (`f32`).
///
/// Returns the elapsed time since the last tick in seconds (`f32`).
pub fn delta() -> f32 {
    unsafe { lotus_script_sys::time::delta_f64() as f32 }
}

/// Gibt die vergangene Zeit seit dem letzten Tick in Sekunden zurück (`f64`).
///
/// Returns the elapsed time since the last tick in seconds (`f64`).
pub fn delta_f64() -> f64 {
    unsafe { lotus_script_sys::time::delta_f64() }
}

/// Gibt die Anzahl der Ticks zurück, seit das Script aktiv ist.
///
/// Returns the number of ticks since the script became active.
pub fn ticks_alive() -> u64 {
    unsafe { lotus_script_sys::time::ticks_alive() }
}

/// Gibt die aktuelle Spielzeit zurück.
///
/// Returns the current in-game time.
pub fn game_time() -> lotus_shared::time::GameTime {
    let unix_micros = unsafe { lotus_script_sys::time::game_time() };
    lotus_shared::time::GameTime::from_unix_micros(unix_micros)
}
