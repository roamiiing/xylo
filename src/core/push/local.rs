use std::{fs, io, path::Path};

use colored::Colorize;
use path_absolutize::Absolutize;

use crate::{config::destinations::local::LocalDestConfig, core::common::DumpMetadata, utils::log};

use super::base::PushStrategy;

pub struct LocalPushStrategy {
    meta: DumpMetadata,
    config: LocalDestConfig,
}

impl LocalPushStrategy {
    pub fn new(meta: &DumpMetadata, config: &LocalDestConfig) -> LocalPushStrategy {
        LocalPushStrategy {
            meta: meta.clone(),
            config: config.clone(),
        }
    }
}

impl PushStrategy for LocalPushStrategy {
    fn push(&self) -> Result<(), io::Error> {
        let archive_path = self.meta.get_archive_path();
        let target_dir = Path::new(&self.config.path).absolutize()?;
        let target_file = target_dir.clone().join(archive_path.file_name().unwrap());

        log::info(
            self.meta.get_service(),
            format!(
                "Pushing to local: {}",
                &target_file.to_string_lossy().bright_cyan()
            ),
        );

        fs::create_dir_all(target_dir)?;
        fs::copy(archive_path, target_file)?;

        Ok(())
    }
}
