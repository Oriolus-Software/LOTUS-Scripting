/// Get the current delta time in seconds.
pub fn delta() -> f32 {
    unsafe { lotus_script_sys::time::delta_f64() as f32 }
}

/// Get the current delta time in seconds.
pub fn delta_f64() -> f64 {
    unsafe { lotus_script_sys::time::delta_f64() }
}

/// Get the number of ticks the script has been alive.
pub fn ticks_alive() -> u64 {
    unsafe { lotus_script_sys::time::ticks_alive() }
}