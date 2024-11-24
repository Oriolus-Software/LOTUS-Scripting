use serde::{Deserialize, Serialize};

/// A unique identifier for a content item.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct ContentId {
    /// The user ID of the content item.
    pub user_id: i32,
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
