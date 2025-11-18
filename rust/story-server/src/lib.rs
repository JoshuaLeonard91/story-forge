pub mod context;
pub mod continuity;
pub mod db;
pub mod error;
pub mod mcp;
pub mod models;
pub mod systems;
pub mod tools;

pub use error::{Result, StoryError};

/// Initialize logging infrastructure
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_library_loads() {
        assert!(true);
    }
}
