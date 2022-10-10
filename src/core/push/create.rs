use crate::{config::destinations::base::DestinationConfig, core::common::DumpMetadata};

use super::{base::PushStrategy, local::LocalPushStrategy};

pub fn create_push_strategy(
    config: &DestinationConfig,
    meta: &DumpMetadata,
) -> Box<dyn PushStrategy> {
    match config {
        DestinationConfig::Local(config) => Box::new(LocalPushStrategy::new(&meta, &config)),
    }
}
