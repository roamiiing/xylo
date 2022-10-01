use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

use crate::utils::hash;

const TMP_DIR: &str = "/tmp/.xylo/";

#[derive(Clone)]
pub struct DumpMetadata {
    service: String,
    hash: String,
    created_at: DateTime<Utc>,
}

impl DumpMetadata {
    pub fn new(service: String) -> DumpMetadata {
        DumpMetadata {
            service,
            hash: hash(),
            created_at: Utc::now(),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        Path::new(TMP_DIR).join(self.to_string())
    }
}

impl ToString for DumpMetadata {
    fn to_string(&self) -> String {
        format!(
            "{}_{}_{}",
            self.service,
            self.created_at.format("%Y-%m-%d-%H-%M-%S"),
            self.hash
        )
    }
}
