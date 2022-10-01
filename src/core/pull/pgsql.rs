use std::{
    fs::{create_dir_all, remove_dir_all, File},
    io::{self, Error},
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
        let command_str = format!(
            "PGPASSWORD={} pg_dumpall -h {} -p {} -U {}",
            self.config
                .password
                .as_ref()
                .map(String::as_str)
                .unwrap_or(""),
            self.config.host,
            self.config.port,
            self.config.username,
        );
        let mut command = create_command(command_str);

        command
            .stdout(file)
            .spawn()
            .and_then(|mut process| process.wait())
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::new(
                        io::ErrorKind::Other,
                        "Unhandled exception during pgdump",
                    ))
                }
            })
    }
}

impl PullStrategy<PgsqlSourceConfig> for PgsqlPullStrategy {
    fn pull(&self) -> Result<DumpMetadata, io::Error> {
        let tmp_path = self.meta.get_path();

        create_dir_all(tmp_path.deref())
            .and_then(|_| File::create(tmp_path.join("pgsql.dump.sql")))
            .and_then(|file| {
                let dump = self.dump(file);

                match dump {
                    Ok(_) => Ok(self.meta.clone()),
                    Err(err) => {
                        remove_dir_all(tmp_path.deref()).unwrap();
                        Err(err)
                    }
                }
            })
    }
}
