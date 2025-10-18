pub mod client;
pub mod config;
pub mod context;
pub mod errors;
pub mod track_event;
pub mod types;

// Generated OpenAPI client
pub mod generated {
    pub mod apis;
    pub mod models;
}

pub use client::TogglrClient;
pub use config::{Config, ConfigBuilder, BackoffConfig};
pub use context::RequestContext;
pub use errors::{TogglrError, TogglrResult};
pub use track_event::{TrackEvent, TrackEventBuilder, EventType, create_track_event};
pub use types::*;
