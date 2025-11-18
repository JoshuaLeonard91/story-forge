use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldRule {
    pub id: Uuid,
    pub story_project_id: Uuid,
    pub name: String,
    pub description: String,
    pub scope: RuleScope,
    pub examples: Option<String>,
    pub keywords: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RuleScope {
    Universal,
    Regional,
    Situational,
}

impl ToString for RuleScope {
    fn to_string(&self) -> String {
        match self {
            RuleScope::Universal => "universal".to_string(),
            RuleScope::Regional => "regional".to_string(),
            RuleScope::Situational => "situational".to_string(),
        }
    }
}

impl RuleScope {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "universal" => Some(RuleScope::Universal),
            "regional" => Some(RuleScope::Regional),
            "situational" => Some(RuleScope::Situational),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_scope_serialization() {
        let scope = RuleScope::Universal;
        let serialized = serde_json::to_string(&scope).unwrap();
        assert_eq!(serialized, "\"universal\"");

        let deserialized: RuleScope = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, RuleScope::Universal);
    }
}
