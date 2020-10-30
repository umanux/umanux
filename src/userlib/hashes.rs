#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
pub struct SourceHash {
    hashvalue: String,
}

impl SourceHash {
    pub fn new(src: &str) -> Self {
        Self {
            hashvalue: src.to_owned(),
        }
    }
    pub fn has_changed(&self, new: &str) -> bool {
        trace!(
            "Old and new lengths: {}, {}",
            self.hashvalue.len(),
            new.len()
        );
        !self.hashvalue.eq(new)
    }
}

pub struct Hashes {
    pub passwd: SourceHash,
    pub shadow: SourceHash,
    pub group: SourceHash,
}

impl Hashes {
    pub fn new(passwd: &str, shadow: &str, group: &str) -> Self {
        Self {
            passwd: SourceHash::new(passwd),
            shadow: SourceHash::new(shadow),
            group: SourceHash::new(group),
        }
    }
}
