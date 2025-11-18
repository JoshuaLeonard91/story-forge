use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, StoryError>;

impl StoryError {
    pub fn not_found(msg: impl Into<String>) -> Self {
        StoryError::NotFound(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        StoryError::ValidationError(msg.into())
    }

    pub fn duplicate(msg: impl Into<String>) -> Self {
        StoryError::DuplicateEntry(msg.into())
    }

    pub fn invalid_state(msg: impl Into<String>) -> Self {
        StoryError::InvalidState(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = StoryError::not_found("Project not found");
        assert!(matches!(err, StoryError::NotFound(_)));

        let err = StoryError::validation("Invalid title");
        assert!(matches!(err, StoryError::ValidationError(_)));
    }

    #[test]
    fn test_error_display() {
        let err = StoryError::not_found("Character not found");
        assert_eq!(err.to_string(), "Not found: Character not found");
    }
}
