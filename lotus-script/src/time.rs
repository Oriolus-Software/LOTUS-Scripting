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

/// Bittet darum, `tick` ab dem nächsten Frame auszulassen, bis sich eine der
/// genannten Script-Variablen ändert oder eine Nachricht eintrifft.
///
/// Der Aufruf kehrt sofort zurück. `late_tick` dieses Frames läuft noch.
/// Mehrere Aufrufe im selben Frame beobachten alle genannten Variablen.
/// [`cancel_idle`] hebt die Bitte wieder auf. Im Weck-Frame ist [`delta`] der
/// Abstand dieses Frames, nicht die Dauer des Schlafs.
///
/// Asks to skip `tick` from the next frame on, until one of the named script
/// variables changes or a message arrives.
///
/// The call returns immediately. This frame's `late_tick` still runs.
/// Repeated calls in the same frame watch every named variable.
/// [`cancel_idle`] withdraws the request. On the wake frame [`delta`] is that
/// frame's step, not the time spent idle.
pub fn idle_until_var_changes(name: &str) {
    let name = lotus_script_sys::FfiObject::new(&name);
    unsafe { lotus_script_sys::time::idle_until_var_changes(name.packed()) }
}

/// Hebt eine Idle-Bitte aus diesem Frame auf.
///
/// Withdraws an idle request made in this frame.
pub fn cancel_idle() {
    unsafe { lotus_script_sys::time::cancel_idle() }
}
