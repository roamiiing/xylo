use std::io;

use crate::utils::command::create_command;

use super::common::DumpMetadata;

pub fn create_archive(meta: &DumpMetadata) -> Result<DumpMetadata, io::Error> {
    let dir_path = meta.get_dir_path();
    let archive_path = meta.get_archive_path();

    create_command(format!(
        "zip -r {} {} -j",
        archive_path.display(),
        dir_path.display()
    ))
    .output()
    .map(|_| meta.clone())
}
