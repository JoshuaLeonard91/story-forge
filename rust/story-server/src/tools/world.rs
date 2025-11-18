use crate::error::{Result, StoryError};
use crate::models::{RuleScope, WorldRule};
use chrono::Utc;
use rusqlite::Connection;
use serde_json::{json, Value};
use uuid::Uuid;

/// Add a new world rule to a story project
pub fn add_world_rule(conn: &Connection, params: Value) -> Result<Value> {
    let project_id_str = params
        .get("projectId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: projectId"))?;

    let project_id = Uuid::parse_str(project_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for projectId"))?;

    let name = params
        .get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: name"))?;

    let description = params
        .get("description")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: description"))?;

    let scope_str = params
        .get("scope")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: scope"))?;

    let scope = RuleScope::from_str(scope_str)
        .ok_or_else(|| StoryError::validation(format!("Invalid scope: {}", scope_str)))?;

    let examples = params.get("examples").and_then(|v| v.as_str());

    // Handle keywords (could be array or string)
    let keywords = params.get("keywords").map(|v| {
        if let Some(arr) = v.as_array() {
            serde_json::to_string(&arr).unwrap()
        } else if let Some(s) = v.as_str() {
            s.to_string()
        } else {
            serde_json::to_string(&v).unwrap()
        }
    });

    // Validate lengths
    if name.len() > 100 {
        return Err(StoryError::validation("Name must be 100 characters or less"));
    }

    let rule = WorldRule {
        id: Uuid::new_v4(),
        story_project_id: project_id,
        name: name.to_string(),
        description: description.to_string(),
        scope: scope.clone(),
        examples: examples.map(|s| s.to_string()),
        keywords,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Insert rule
    conn.execute(
        "INSERT INTO world_rules (id, story_project_id, name, description, scope, examples, keywords, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            rule.id.to_string(),
            rule.story_project_id.to_string(),
            &rule.name,
            &rule.description,
            rule.scope.to_string(),
            &rule.examples,
            &rule.keywords,
            rule.created_at.to_rfc3339(),
            rule.updated_at.to_rfc3339(),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            StoryError::duplicate(format!("World rule '{}' already exists in this project", name))
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    log::info!("Created world rule: {} ({})", rule.name, rule.id);

    Ok(json!({
        "ruleId": rule.id.to_string(),
        "name": rule.name,
        "description": rule.description,
        "scope": rule.scope.to_string(),
        "examples": rule.examples,
        "keywords": rule.keywords.as_ref().and_then(|k| serde_json::from_str::<Value>(k).ok()),
        "createdAt": rule.created_at.to_rfc3339()
    }))
}

/// Get a world rule by ID
pub fn get_world_rule(conn: &Connection, params: Value) -> Result<Value> {
    let rule_id_str = params
        .get("ruleId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: ruleId"))?;

    let rule_id = Uuid::parse_str(rule_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for ruleId"))?;

    let mut stmt = conn.prepare(
        "SELECT id, story_project_id, name, description, scope, examples, keywords, created_at, updated_at
         FROM world_rules WHERE id = ?1"
    )?;

    let rule = stmt.query_row([rule_id.to_string()], |row| {
        Ok(WorldRule {
            id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
            story_project_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
            name: row.get(2)?,
            description: row.get(3)?,
            scope: RuleScope::from_str(&row.get::<_, String>(4)?).unwrap(),
            examples: row.get(5)?,
            keywords: row.get(6)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    }).map_err(|e| {
        if e == rusqlite::Error::QueryReturnedNoRows {
            StoryError::not_found(format!("World rule not found: {}", rule_id))
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    Ok(json!({
        "ruleId": rule.id.to_string(),
        "projectId": rule.story_project_id.to_string(),
        "name": rule.name,
        "description": rule.description,
        "scope": rule.scope.to_string(),
        "examples": rule.examples,
        "keywords": rule.keywords.as_ref().and_then(|k| serde_json::from_str::<Value>(k).ok())
    }))
}

/// List all world rules in a project
pub fn list_world_rules(conn: &Connection, params: Value) -> Result<Value> {
    let project_id_str = params
        .get("projectId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: projectId"))?;

    let project_id = Uuid::parse_str(project_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for projectId"))?;

    let mut stmt = conn.prepare(
        "SELECT id, name, description, scope, keywords
         FROM world_rules
         WHERE story_project_id = ?1
         ORDER BY scope, name"
    )?;

    let rules = stmt
        .query_map([project_id.to_string()], |row| {
            let keywords_str: Option<String> = row.get(4)?;
            Ok(json!({
                "ruleId": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "description": row.get::<_, String>(2)?,
                "scope": row.get::<_, String>(3)?,
                "keywords": keywords_str.as_ref().and_then(|k| serde_json::from_str::<Value>(k).ok())
            }))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    log::info!("Listed {} world rules for project {}", rules.len(), project_id);

    Ok(json!({
        "rules": rules
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::tools::project::create_story_project;
    use tempfile::tempdir;

    #[test]
    fn test_add_world_rule_success() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        let params = json!({
            "projectId": project_id,
            "name": "Magic System",
            "description": "Magic requires mana",
            "scope": "universal",
            "keywords": ["magic", "mana"]
        });

        let result = add_world_rule(&conn, params);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("ruleId").is_some());
        assert_eq!(response.get("name").unwrap(), "Magic System");
        assert_eq!(response.get("scope").unwrap(), "universal");
    }

    #[test]
    fn test_list_world_rules() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        add_world_rule(&conn, json!({"projectId": project_id, "name": "Rule 1", "description": "Test", "scope": "universal"})).unwrap();
        add_world_rule(&conn, json!({"projectId": project_id, "name": "Rule 2", "description": "Test", "scope": "regional"})).unwrap();

        let result = list_world_rules(&conn, json!({"projectId": project_id}));
        assert!(result.is_ok());

        let response = result.unwrap();
        let rules = response.get("rules").unwrap().as_array().unwrap();
        assert_eq!(rules.len(), 2);
    }
}
