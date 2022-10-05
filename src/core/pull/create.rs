use crate::config::sources::base::SourceConfig;

use super::{base::PullStrategy, pgsql::PgsqlPullStrategy};

pub fn create_pull_strategy<S: Into<String>>(
    service: S,
    config: &SourceConfig,
) -> Box<dyn PullStrategy> {
    match config {
        SourceConfig::Pgsql(config) => Box::new(PgsqlPullStrategy::new(service, &config)),
    }
}
