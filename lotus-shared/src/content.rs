//! Content-Identifikatoren, die zwischen Scripts und Engine geteilt werden.
//!
//! Content identifiers shared between scripts and the engine.

use serde::{Deserialize, Serialize};

/// Eindeutiger Identifikator für ein Content-Element.
///
/// A unique identifier for a content item.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ContentId {
    /// User-ID des Content-Elements.
    ///
    /// The user ID of the content item.
    pub user_id: i32,
    /// Sub-ID des Content-Elements.
    ///
    /// The sub ID of the content item.
    pub sub_id: i32,
}

impl Eq for ContentId {}

impl std::hash::Hash for ContentId {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
        self.sub_id.hash(state);
    }
}

#[cfg(feature = "ffi")]
mod ffi {
    use lotus_script_sys::{FfiObject, FromFfi};

    impl FromFfi for crate::content::ContentId {
        type FfiType = u64;
        fn from_ffi(ffi: Self::FfiType) -> Self {
            FfiObject::from_packed(ffi).deserialize()
        }
    }
}
