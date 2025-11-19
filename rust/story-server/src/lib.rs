pub mod context;
pub mod continuity;
pub mod db;
pub mod error;
pub mod mcp;
pub mod models;
pub mod systems;
pub mod tools;

pub use error::{Result, StoryError};

/// Initialize logging infrastructure with file and console output
pub fn init_logging() {
    use std::env;
    
    let log_level = env::var("RUST_LOG")
        .unwrap_or_else(|_| "info".to_string())
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    // Set up logging to both file and stderr
    let mut dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());
    
    // Try to add file logging
    if let Ok(log_file) = fern::log_file("story-server.log") {
        dispatch = dispatch.chain(log_file);
    } else {
        eprintln!("Warning: Could not create log file, logging to stderr only");
    }
    
    if let Err(e) = dispatch.apply() {
        eprintln!("Failed to initialize logging: {}", e);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_library_loads() {
        assert!(true);
    }
}
