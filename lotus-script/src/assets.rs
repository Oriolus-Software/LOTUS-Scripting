use lotus_script_sys::FfiObject;
use lotus_shared::content::ContentId;

/// Preload an asset like a texture you want to swap later or font you want to use.
/// The asset will be loaded and kept in memory until the script gets unloaded.
pub fn preload(id: ContentId) {
    let id = FfiObject::new(&id);

    unsafe {
        lotus_script_sys::assets::preload(id.packed());
    }
}
