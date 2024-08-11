use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct ContentId {
    pub user_id: i32,
    pub sub_id: i32,
    pub version: f64,
}

impl Eq for ContentId {}

impl std::hash::Hash for ContentId {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.user_id.hash(state);
        self.sub_id.hash(state);
        self.version.to_bits().hash(state);
    }
}
