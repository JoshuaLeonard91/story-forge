use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub title: Option<String>,
    pub position: i32,
    pub location: Option<String>,
    pub time_description: Option<String>,
    pub content: String,
    pub word_count: i32,
    pub status: SceneStatus,
    pub scene_outline: Option<String>,
    pub ai_generated: bool,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SceneStatus {
    Planned,
    Draft,
    Complete,
    NeedsRevision,
}

impl ToString for SceneStatus {
    fn to_string(&self) -> String {
        match self {
            SceneStatus::Planned => "planned".to_string(),
            SceneStatus::Draft => "draft".to_string(),
            SceneStatus::Complete => "complete".to_string(),
            SceneStatus::NeedsRevision => "needs_revision".to_string(),
        }
    }
}

impl SceneStatus {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "planned" => Some(SceneStatus::Planned),
            "draft" => Some(SceneStatus::Draft),
            "complete" => Some(SceneStatus::Complete),
            "needs_revision" => Some(SceneStatus::NeedsRevision),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotStructure {
    pub id: Uuid,
    pub story_project_id: Uuid,
    pub structure_type: StructureType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StructureType {
    ThreeAct,
    FiveAct,
    HeroJourney,
    Custom,
}

impl ToString for StructureType {
    fn to_string(&self) -> String {
        match self {
            StructureType::ThreeAct => "three_act".to_string(),
            StructureType::FiveAct => "five_act".to_string(),
            StructureType::HeroJourney => "hero_journey".to_string(),
            StructureType::Custom => "custom".to_string(),
        }
    }
}

impl StructureType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "three_act" => Some(StructureType::ThreeAct),
            "five_act" => Some(StructureType::FiveAct),
            "hero_journey" => Some(StructureType::HeroJourney),
            "custom" => Some(StructureType::Custom),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scene_status_serialization() {
        let status = SceneStatus::Draft;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"draft\"");
    }

    #[test]
    fn test_structure_type_from_str() {
        assert_eq!(StructureType::from_str("three_act"), Some(StructureType::ThreeAct));
        assert_eq!(StructureType::from_str("hero_journey"), Some(StructureType::HeroJourney));
        assert_eq!(StructureType::from_str("invalid"), None);
    }
}
