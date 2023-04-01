pub mod sources {
    pub mod base;
    pub mod pgsql;
}

pub mod destinations {
    pub mod base;
    pub mod local;
    pub mod s3;
    pub mod scp;
}

pub mod base;

pub mod common;

pub use base::parse_config;
