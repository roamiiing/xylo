mod sources {
    pub mod base;
    pub mod pgsql;
}

mod destinations {
    pub mod base;
    pub mod gdrive;
    pub mod local;
}

pub mod base;

pub use base::parse_config;
