use std::{
    fs,
    io::{self, Write},
    net::TcpStream,
    path::Path,
};

use colored::Colorize;
use ssh2::Session;

use crate::{
    config::destinations::scp::{ScpDestConfig, ScpDestUnpackedConfig},
    core::common::DumpMetadata,
    utils::log,
};

use super::base::PushStrategy;

pub struct ScpPushStrategy {
    meta: DumpMetadata,
    config: ScpDestConfig,
}

impl ScpPushStrategy {
    pub fn new(meta: &DumpMetadata, config: &ScpDestConfig) -> ScpPushStrategy {
        ScpPushStrategy {
            meta: meta.clone(),
            config: config.clone(),
        }
    }
}

impl PushStrategy for ScpPushStrategy {
    fn push(&self) -> Result<(), io::Error> {
        let ScpDestUnpackedConfig {
            host,
            port,
            user,
            password,
            path,
        } = self.config.unpack().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Failed to unpack SCP config. Check your config file",
            )
        })?;

        let archive_path = self.meta.get_archive_path();
        let target_path = Path::new(&path).join(archive_path.file_name().unwrap());

        log::info(self.meta.get_service(), "Pushing via SCP");
        log::info(
            self.meta.get_service(),
            format!("Host: {}", &host.bright_cyan()),
        );
        log::info(
            self.meta.get_service(),
            format!("Path: {}", &target_path.to_string_lossy().bright_cyan()),
        );

        let archive_content = fs::read(archive_path)?;

        // TODO: move this to a separate module and reuse it in SSH pull strategy
        let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        sess.userauth_password(&user, &password)?;

        let mut remote_file = sess.scp_send(
            target_path.as_ref(),
            0o644,
            archive_content.len() as u64,
            None,
        )?;
        remote_file.write_all(&archive_content)?;

        Ok(())
    }
}
