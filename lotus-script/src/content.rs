//! Laden und Vorhalten von Content-Assets.
//!
//! Loading and preloading content assets.

use lotus_script_sys::FfiObject;
pub use lotus_shared::content::*;

/// Lädt ein Asset vor (z. B. Textur oder Schrift) und hält es im Speicher,
/// bis das Script entladen wird.
///
/// Preloads an asset (e.g. texture or font) and keeps it in memory until the script is unloaded.
pub fn preload(id: ContentId) {
    let id = FfiObject::new(&id);

    unsafe {
        lotus_script_sys::assets::preload(id.packed());
    }
}
