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
/// Zusammen mit [`idle_for`] wacht das Script beim ersten Ereignis auf:
/// Variablenänderung, Nachricht oder Ablauf der Zeitspanne.
///
/// Asks to skip `tick` from the next frame on, until one of the named script
/// variables changes or a message arrives.
///
/// The call returns immediately. This frame's `late_tick` still runs.
/// Repeated calls in the same frame watch every named variable.
/// [`cancel_idle`] withdraws the request. On the wake frame [`delta`] is that
/// frame's step, not the time spent idle.
///
/// Combined with [`idle_for`], the script wakes on the first of: a variable
/// change, a message, or the deadline.
pub fn idle_until_var_changes(name: &str) {
    let name = lotus_script_sys::FfiObject::new(&name);
    unsafe { lotus_script_sys::time::idle_until_var_changes(name.packed()) }
}

/// Bittet darum, `tick` auszulassen, bis eine Simulationszeit zwischen
/// `min_seconds` und `max_seconds` verstrichen ist.
///
/// Der Simulator wählt die Dauer einmalig gleichverteilt aus `[min, max)`.
/// Gleiche Grenzen sind ein festes Intervall und ziehen keine Zufallszahl.
/// Sonst kommt die Zahl aus dem Script-Zufallsgenerator (`rand`). Die Zeit
/// läuft mit der Simulation, Pause hält den Wecker an, und es gilt dieselbe
/// Schrittweite wie bei [`delta`].
///
/// Die Spanne gilt zusätzlich zu [`idle_until_var_changes`]: das Script wacht
/// beim ersten Ereignis auf, also bei Variablenänderung, Nachricht oder Ablauf
/// der Zeit. Ohne Variablenbeobachtung bleiben Zeitspanne und Nachrichten die
/// Weckgründe. Mehrere Aufrufe im selben Frame: die letzte Spanne gilt.
/// [`cancel_idle`] hebt auch die Zeitspanne auf.
///
/// Der Aufruf kehrt sofort zurück. `late_tick` dieses Frames läuft noch.
/// Im Weck-Frame ist [`delta`] der Abstand dieses Frames, nicht die Schlafdauer.
/// Liegt die gewählte Dauer unter einem Frame, läuft der nächste `tick` noch.
///
/// `NaN` und unendlich werden ignoriert. Negative Grenzen zählen als 0.
/// Vertauschte Grenzen werden getauscht.
///
/// Asks to skip `tick` until a simulation time between `min_seconds` and
/// `max_seconds` has elapsed.
///
/// The simulator picks the duration once, uniformly from `[min, max)`.
/// Equal bounds are a fixed interval and do not draw a random number.
/// Otherwise the draw comes from the script RNG (`rand`). Time follows the
/// simulation, pause holds the alarm, and the step is the same as [`delta`].
///
/// The span combines with [`idle_until_var_changes`]: the script wakes on the
/// first of a variable change, a message, or the deadline. Without a variable
/// watch, the deadline and messages are the wake reasons. Repeated calls in
/// the same frame: the last span wins. [`cancel_idle`] also clears the span.
///
/// The call returns immediately. This frame's `late_tick` still runs.
/// On the wake frame [`delta`] is that frame's step, not the time spent idle.
/// A duration shorter than one frame still runs the next `tick`.
///
/// `NaN` and infinities are ignored. Negative bounds count as 0.
/// Swapped bounds are swapped back.
pub fn idle_for(min_seconds: f64, max_seconds: f64) {
    unsafe { lotus_script_sys::time::idle_for(min_seconds, max_seconds) }
}

/// Hebt eine Idle-Bitte aus diesem Frame auf, einschließlich einer Zeitspanne
/// von [`idle_for`].
///
/// Withdraws an idle request made in this frame, including an [`idle_for`] span.
pub fn cancel_idle() {
    unsafe { lotus_script_sys::time::cancel_idle() }
}
