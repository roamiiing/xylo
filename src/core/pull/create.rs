use crate::config::sources::base::SourceConfig;

use super::{base::PullStrategy, local::LocalPullStrategy, pgsql::PgsqlPullStrategy};

pub fn create_pull_strategy<S: Into<String>>(
    service: S,
    config: &SourceConfig,
) -> Box<dyn PullStrategy> {
    match config {
        SourceConfig::Local(config) => Box::new(LocalPullStrategy::new(service, config)),
        SourceConfig::Pgsql(config) => Box::new(PgsqlPullStrategy::new(service, config)),
    }
}
