//! Informationen zum Modul-Slot, auf dem dieses Script läuft.
//!
//! Information about the module slot this script is running on.

fn on_negative_to_none(index: i32) -> Option<i32> {
    match index {
        -1 => None,
        i => Some(i),
    }
}

/// Gibt den Cockpit-Index des Modul-Slots zurück, auf dem dieses Modul sitzt.
/// `None`, wenn das Script nicht für ein Modul läuft.
///
/// Returns the cockpit index of the module slot this module is on.
/// `None` if this script is not running for a module.
pub fn module_slot_cockpit_index() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::env::module_slot_cockpit_index() })
}

/// Gibt den Index des Modul-Slots innerhalb der Klassengruppe zurück.
/// `None`, wenn das Script nicht für ein Modul läuft.
///
/// Returns the index of the module slot within its class group.
/// `None` if this script is not running for a module.
pub fn module_slot_index_in_class_group() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::env::module_slot_index_in_class_group() })
}

/// Gibt den Index des Modul-Slots zurück, auf dem dieses Modul sitzt.
/// `None`, wenn das Script nicht für ein Modul läuft.
///
/// Returns the index of the module slot this module is on.
/// `None` if this script is not running for a module.
pub fn module_slot_index() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::env::module_slot_index() })
}
