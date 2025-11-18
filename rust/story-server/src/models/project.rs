use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryProject {
    pub id: Uuid,
    pub title: String,
    pub genre: Option<String>,
    pub intended_length: ProjectLength,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub word_count: i32,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectLength {
    ShortStory,
    Novella,
    Novel,
    Series,
}

impl ToString for ProjectLength {
    fn to_string(&self) -> String {
        match self {
            ProjectLength::ShortStory => "short_story".to_string(),
            ProjectLength::Novella => "novella".to_string(),
            ProjectLength::Novel => "novel".to_string(),
            ProjectLength::Series => "series".to_string(),
        }
    }
}

impl ProjectLength {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "short_story" => Some(ProjectLength::ShortStory),
            "novella" => Some(ProjectLength::Novella),
            "novel" => Some(ProjectLength::Novel),
            "series" => Some(ProjectLength::Series),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Draft,
    InProgress,
    Complete,
    Archived,
}

impl ToString for ProjectStatus {
    fn to_string(&self) -> String {
        match self {
            ProjectStatus::Draft => "draft".to_string(),
            ProjectStatus::InProgress => "in_progress".to_string(),
            ProjectStatus::Complete => "complete".to_string(),
            ProjectStatus::Archived => "archived".to_string(),
        }
    }
}

impl ProjectStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(ProjectStatus::Draft),
            "in_progress" => Some(ProjectStatus::InProgress),
            "complete" => Some(ProjectStatus::Complete),
            "archived" => Some(ProjectStatus::Archived),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_length_serialization() {
        let length = ProjectLength::Novel;
        let serialized = serde_json::to_string(&length).unwrap();
        assert_eq!(serialized, "\"novel\"");

        let deserialized: ProjectLength = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, ProjectLength::Novel);
    }

    #[test]
    fn test_project_status_from_str() {
        assert_eq!(ProjectStatus::from_str("draft"), Some(ProjectStatus::Draft));
        assert_eq!(ProjectStatus::from_str("in_progress"), Some(ProjectStatus::InProgress));
        assert_eq!(ProjectStatus::from_str("invalid"), None);
    }
}
