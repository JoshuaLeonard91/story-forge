use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: Uuid,
    pub story_project_id: Uuid,
    pub name: String,
    pub role: CharacterRole,
    pub personality_traits: Option<String>,
    pub physical_description: Option<String>,
    pub backstory: Option<String>,
    pub current_state: Option<String>,
    pub first_appearance_scene_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CharacterRole {
    Protagonist,
    Antagonist,
    Supporting,
    Minor,
}

impl ToString for CharacterRole {
    fn to_string(&self) -> String {
        match self {
            CharacterRole::Protagonist => "protagonist".to_string(),
            CharacterRole::Antagonist => "antagonist".to_string(),
            CharacterRole::Supporting => "supporting".to_string(),
            CharacterRole::Minor => "minor".to_string(),
        }
    }
}

impl CharacterRole {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "protagonist" => Some(CharacterRole::Protagonist),
            "antagonist" => Some(CharacterRole::Antagonist),
            "supporting" => Some(CharacterRole::Supporting),
            "minor" => Some(CharacterRole::Minor),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRelationship {
    pub id: Uuid,
    pub source_character_id: Uuid,
    pub target_character_id: Uuid,
    pub relationship_type: RelationshipType,
    pub description: Option<String>,
    pub strength: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipType {
    Ally,
    Enemy,
    Family,
    Romantic,
    Mentor,
    Rival,
    Neutral,
    Unknown,
}

impl ToString for RelationshipType {
    fn to_string(&self) -> String {
        match self {
            RelationshipType::Ally => "ally".to_string(),
            RelationshipType::Enemy => "enemy".to_string(),
            RelationshipType::Family => "family".to_string(),
            RelationshipType::Romantic => "romantic".to_string(),
            RelationshipType::Mentor => "mentor".to_string(),
            RelationshipType::Rival => "rival".to_string(),
            RelationshipType::Neutral => "neutral".to_string(),
            RelationshipType::Unknown => "unknown".to_string(),
        }
    }
}

impl RelationshipType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ally" => Some(RelationshipType::Ally),
            "enemy" => Some(RelationshipType::Enemy),
            "family" => Some(RelationshipType::Family),
            "romantic" => Some(RelationshipType::Romantic),
            "mentor" => Some(RelationshipType::Mentor),
            "rival" => Some(RelationshipType::Rival),
            "neutral" => Some(RelationshipType::Neutral),
            "unknown" => Some(RelationshipType::Unknown),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_role_serialization() {
        let role = CharacterRole::Protagonist;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"protagonist\"");
    }

    #[test]
    fn test_relationship_type_from_str() {
        assert_eq!(RelationshipType::from_str("ally"), Some(RelationshipType::Ally));
        assert_eq!(RelationshipType::from_str("enemy"), Some(RelationshipType::Enemy));
        assert_eq!(RelationshipType::from_str("invalid"), None);
    }
}
