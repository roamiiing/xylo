use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io,
    ops::Deref,
};

use super::base::PullStrategy;
use crate::{
    config::sources::pgsql::PgsqlSourceConfig, core::common::DumpMetadata,
    utils::command::create_command,
};

pub struct PgsqlPullStrategy {
    meta: DumpMetadata,
    config: PgsqlSourceConfig,
}

impl PgsqlPullStrategy {
    pub fn new<S: Into<String>>(service: S, config: &PgsqlSourceConfig) -> PgsqlPullStrategy {
        PgsqlPullStrategy {
            meta: DumpMetadata::new(service.into()),
            config: config.clone(),
        }
    }

    fn dump(&self, file: File) -> Result<(), io::Error> {
        let unpacked_config = self.config.unpack().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Not enough data provided")
        })?;

        let command_str = format!(
            "PGPASSWORD={} pg_dumpall -h {} -p {} -U {}",
            unpacked_config.password,
            unpacked_config.host,
            unpacked_config.port,
            unpacked_config.username,
        );

        let mut command = create_command(command_str);

        let status = command
            .stdout(file)
            .spawn()
            .and_then(|mut process| process.wait())?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Unhandled exception during pgdump",
            ))
        }
    }
}

impl PullStrategy for PgsqlPullStrategy {
    fn pull(&self) -> Result<DumpMetadata, io::Error> {
        let tmp_path = self.meta.get_dir_path();

        create_dir_all(tmp_path.deref())?;
        let file = File::create(tmp_path.join("pgsql.dump.sql"))?;

        let dump = self.dump(file);

        match dump {
            Ok(_) => Ok(self.meta.clone()),
            Err(err) => {
                remove_dir_all(tmp_path.deref()).unwrap();
                Err(err)
            }
        }
    }
}
