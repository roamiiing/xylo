use std::{fs, io, path::Path};

use colored::Colorize;
use s3::{bucket::Bucket, creds::Credentials, Region};

use crate::{
    config::destinations::s3::{S3DestConfig, S3DestUnpackedConfig},
    core::common::DumpMetadata,
    utils::log,
};

use super::base::PushStrategy;

pub struct S3PushStrategy {
    meta: DumpMetadata,
    config: S3DestConfig,
}

impl S3PushStrategy {
    pub fn new(meta: &DumpMetadata, config: &S3DestConfig) -> S3PushStrategy {
        S3PushStrategy {
            meta: meta.clone(),
            config: config.clone(),
        }
    }
}

impl PushStrategy for S3PushStrategy {
    fn push(&self) -> Result<(), io::Error> {
        let S3DestUnpackedConfig {
            endpoint,
            access_key,
            secret_key,
            region,
            bucket,
            path,
        } = self.config.unpack().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Failed to unpack S3 config. Check your config file",
            )
        })?;

        let archive_path = self.meta.get_archive_path();
        let target_path = Path::new(&path).join(archive_path.file_name().unwrap());

        let s3_bucket = Bucket::new(
            &bucket,
            Region::Custom { region, endpoint },
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).map_err(
                |e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to create S3 credentials: {}", e),
                    )
                },
            )?,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        log::info(self.meta.get_service(), "Pushing to S3");
        log::info(
            self.meta.get_service(),
            format!("Bucket name: {}", &bucket.bright_cyan()),
        );
        log::info(
            self.meta.get_service(),
            format!("Path: {}", &target_path.to_string_lossy().bright_cyan()),
        );

        let archive_content = fs::read(archive_path)?;

        s3_bucket
            .put_object_blocking(target_path.to_string_lossy(), &archive_content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok(())
    }
}
