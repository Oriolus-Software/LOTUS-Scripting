fn on_negative_to_none(index: i32) -> Option<i32> {
    match index {
        -1 => None,
        i => Some(i),
    }
}

/// Returns the index of the cockpit index of the module slot on which this module is set.
/// None if this script is not running for a module.
pub fn module_slot_cockpit_index() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::module::module_slot_cockpit_index() })
}

/// Returns the index of the module slot in the class group of the module slot on which this module is set.
/// None if this script is not running for a module.
pub fn module_slot_index_in_class_group() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::module::module_slot_index_in_class_group() })
}

/// Returns the index of the module slot on which this module is set.
/// None if this script is not running for a module.
pub fn module_slot_index() -> Option<i32> {
    on_negative_to_none(unsafe { lotus_script_sys::module::module_slot_index() })
}
