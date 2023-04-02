use std::io;

use crate::utils::command::create_command;

use super::common::DumpMetadata;

pub fn create_archive(meta: &DumpMetadata) -> Result<(), io::Error> {
    let dir_path = meta.get_dir_path();
    let archive_path = meta.get_archive_path();

    // TODO: use zip/tar library?
    let output = create_command(format!(
        "zip -r {} {}",
        archive_path.display(),
        dir_path.display()
    ))
    .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Unhandled error during archive creation:\n{}",
                String::from_utf8_lossy(&output.stdout)
            ),
        ))
    }
}
