use std::io::Error;

use crate::core::common::DumpMetadata;

pub trait PullStrategy {
    fn pull(&self) -> Result<DumpMetadata, Error>;
}
