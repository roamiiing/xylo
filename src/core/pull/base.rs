use std::io::Error;

use crate::core::common::DumpMetadata;

pub trait PullStrategy<T> {
    fn pull(&self) -> Result<&DumpMetadata, Error>;
}
