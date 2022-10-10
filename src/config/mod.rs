pub mod sources {
    pub mod base;
    pub mod pgsql;
}

pub mod destinations {
    pub mod base;
    pub mod local;
}

pub mod base;

pub mod common;

pub use base::parse_config;
