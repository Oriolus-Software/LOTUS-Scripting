//! Zufallszahlen-Generierung für Scripts.
//!
//! Random number generation for scripts.

use std::ops::{Bound, RangeBounds};

/// Setzt den Zufallsgenerator mit einem zufälligen Startwert.
///
/// Seeds the random number generator with a random seed.
pub fn random_seed() {
    unsafe { lotus_script_sys::rand::random_seed() }
}

/// Setzt den Zufallsgenerator mit dem angegebenen Startwert.
///
/// Seeds the random number generator with the given seed.
pub fn seed(seed: u64) {
    unsafe { lotus_script_sys::rand::seed(seed) }
}

/// Erzeugt eine Zufallszahl `f64` im Bereich 0 bis 1.
///
/// Generates a random `f64` in the range 0 to 1.
pub fn gen_f64() -> f64 {
    unsafe { lotus_script_sys::rand::f64() }
}

/// Erzeugt eine Zufallszahl `u64` im angegebenen Bereich.
///
/// Generates a random `u64` for the given range.
pub fn gen_u64(range: impl RangeBounds<u64>) -> u64 {
    let min = match range.start_bound() {
        Bound::Included(min) => *min,
        Bound::Excluded(min) => min + 1,
        Bound::Unbounded => 0,
    };

    let max = match range.end_bound() {
        Bound::Included(max) => *max,
        Bound::Excluded(max) => max - 1,
        Bound::Unbounded => u64::MAX,
    };

    assert!(min <= max, "min must be less than or equal to max");

    unsafe { lotus_script_sys::rand::u64(min, max) }
}
