use std::{
    fs::{create_dir_all, remove_dir_all},
    io,
    ops::Deref,
};

use super::base::PullStrategy;
use crate::{
    config::sources::local::LocalSourceConfig, core::common::DumpMetadata,
    utils::command::create_command,
};

pub struct LocalPullStrategy {
    meta: DumpMetadata,
    config: LocalSourceConfig,
}

impl LocalPullStrategy {
    pub fn new<S: Into<String>>(service: S, config: &LocalSourceConfig) -> LocalPullStrategy {
        LocalPullStrategy {
            meta: DumpMetadata::new(service.into()),
            config: config.clone(),
        }
    }
}

impl PullStrategy for LocalPullStrategy {
    fn pull(&self) -> Result<DumpMetadata, io::Error> {
        let config = self.config.unpack().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Not enough data provided")
        })?;

        let tmp_path = self.meta.get_dir_path();

        create_dir_all(tmp_path.deref())?;

        // TODO - use copy from std::fs
        let status = create_command(format!(
            "cp -r {} {}",
            config.path,
            tmp_path.to_string_lossy()
        ))
        .spawn()?
        .wait()?;

        if status.success() {
            Ok(self.meta.clone())
        } else {
            remove_dir_all(tmp_path.deref())?;
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Unhandled exception during local pull",
            ))
        }
    }
}
