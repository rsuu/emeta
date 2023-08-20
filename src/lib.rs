pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0";

pub type Res<T> = anyhow::Result<T>;

pub mod api;
pub mod cli;
pub mod meta;
pub mod utils;

// struct/enum/mod
pub use ureq::Request;
pub use {
    api::{anilist, mangadex},
    cli::{App, Cli},
    meta::{MediaType, Site, WatchStatus},
    utils::{Editor, Term},
};

// trait
pub use api::Query;

// macro
pub use {
    anyhow::bail,
    log::{debug, info},
};
