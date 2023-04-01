use crate::{config::destinations::base::DestinationConfig, core::common::DumpMetadata};

use super::{
    base::PushStrategy, local::LocalPushStrategy, s3::S3PushStrategy, scp::ScpPushStrategy,
};

pub fn create_push_strategy(
    config: &DestinationConfig,
    meta: &DumpMetadata,
) -> Box<dyn PushStrategy> {
    match config {
        DestinationConfig::Local(config) => Box::new(LocalPushStrategy::new(meta, config)),
        DestinationConfig::S3(config) => Box::new(S3PushStrategy::new(meta, config)),
        DestinationConfig::Scp(config) => Box::new(ScpPushStrategy::new(meta, config)),
    }
}
