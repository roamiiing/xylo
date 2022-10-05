use crate::{config::destinations::base::DestinationConfig, core::common::DumpMetadata};

use super::{base::PushStrategy, gdrive::GoogleDrivePushStrategy, local::LocalPushStrategy};

pub fn create_push_strategy(
    config: &DestinationConfig,
    meta: &DumpMetadata,
) -> Box<dyn PushStrategy> {
    match config {
        DestinationConfig::Local(config) => Box::new(LocalPushStrategy::new(&meta, &config)),
        DestinationConfig::GoogleDrive(config) => {
            Box::new(GoogleDrivePushStrategy::new(&meta, &config))
        }
    }
}
