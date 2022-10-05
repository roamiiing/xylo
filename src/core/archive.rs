use std::io;

use crate::utils::command::create_command;

use super::common::DumpMetadata;

pub fn create_archive(meta: &DumpMetadata) -> Result<(), io::Error> {
    let dir_path = meta.get_dir_path();
    let archive_path = meta.get_archive_path();

    // TODO: use zip/tar library?
    create_command(format!(
        "zip -r {} {} -j",
        archive_path.display(),
        dir_path.display()
    ))
    .output()?;

    Ok(())
}
