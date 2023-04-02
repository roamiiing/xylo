use std::{
    fs, io,
    path::{Path, PathBuf},
};

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

    pub fn get_dir_path(&self) -> PathBuf {
        Path::new(TMP_DIR).join(self.to_string())
    }

    pub fn get_archive_path(&self) -> PathBuf {
        let mut path = self.get_dir_path();
        path.set_extension("zip");
        path
    }

    pub fn get_service(&self) -> String {
        self.service.clone()
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

pub fn cleanup(meta: &DumpMetadata) -> Result<(), io::Error> {
    let dir_path = meta.get_dir_path();
    let archive_path = meta.get_archive_path();

    fs::remove_dir_all(dir_path)?;
    fs::remove_file(archive_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup() {
        let meta = DumpMetadata::new("test".to_string());
        let dir_path = meta.get_dir_path();
        let archive_path = meta.get_archive_path();

        fs::create_dir_all(dir_path.clone()).unwrap();
        fs::File::create(archive_path.clone()).unwrap();

        assert!(dir_path.exists());
        assert!(archive_path.exists());

        cleanup(&meta).unwrap();

        assert!(!dir_path.exists());
        assert!(!archive_path.exists());
    }
}
