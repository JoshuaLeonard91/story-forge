use crate::error::{Result, StoryError};
use crate::models::{Character, CharacterRelationship, CharacterRole, RelationshipType};
use chrono::Utc;
use rusqlite::Connection;
use serde_json::{json, Value};
use uuid::Uuid;

/// Add a new character to a story project
pub fn add_character(conn: &Connection, params: Value) -> Result<Value> {
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

    let role_str = params
        .get("role")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: role"))?;

    let role = CharacterRole::from_str(role_str)
        .ok_or_else(|| StoryError::validation(format!("Invalid role: {}", role_str)))?;

    let personality_traits = params.get("personalityTraits").and_then(|v| v.as_str());
    let physical_description = params.get("physicalDescription").and_then(|v| v.as_str());
    let backstory = params.get("backstory").and_then(|v| v.as_str());
    let current_state = params.get("currentState").and_then(|v| v.as_str());

    // Validate name length
    if name.len() > 100 {
        return Err(StoryError::validation("Name must be 100 characters or less"));
    }

    let character = Character {
        id: Uuid::new_v4(),
        story_project_id: project_id,
        name: name.to_string(),
        role: role.clone(),
        personality_traits: personality_traits.map(|s| s.to_string()),
        physical_description: physical_description.map(|s| s.to_string()),
        backstory: backstory.map(|s| s.to_string()),
        current_state: current_state.map(|s| s.to_string()),
        first_appearance_scene_id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Insert character
    conn.execute(
        "INSERT INTO characters (id, story_project_id, name, role, personality_traits, physical_description, backstory, current_state, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            character.id.to_string(),
            character.story_project_id.to_string(),
            &character.name,
            character.role.to_string(),
            &character.personality_traits,
            &character.physical_description,
            &character.backstory,
            &character.current_state,
            character.created_at.to_rfc3339(),
            character.updated_at.to_rfc3339(),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            StoryError::duplicate(format!("Character '{}' already exists in this project", name))
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    log::info!("Created character: {} ({})", character.name, character.id);

    Ok(json!({
        "characterId": character.id.to_string(),
        "name": character.name,
        "role": character.role.to_string(),
        "personalityTraits": character.personality_traits,
        "physicalDescription": character.physical_description,
        "backstory": character.backstory,
        "currentState": character.current_state,
        "createdAt": character.created_at.to_rfc3339()
    }))
}

/// Get a character by ID
pub fn get_character(conn: &Connection, params: Value) -> Result<Value> {
    let character_id_str = params
        .get("characterId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: characterId"))?;

    let character_id = Uuid::parse_str(character_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for characterId"))?;

    let mut stmt = conn.prepare(
        "SELECT id, story_project_id, name, role, personality_traits, physical_description, backstory, current_state, created_at, updated_at
         FROM characters WHERE id = ?1"
    )?;

    let character = stmt.query_row([character_id.to_string()], |row| {
        Ok(Character {
            id: Uuid::parse_str(row.get::<_, String>(0)?.as_str()).unwrap(),
            story_project_id: Uuid::parse_str(row.get::<_, String>(1)?.as_str()).unwrap(),
            name: row.get(2)?,
            role: CharacterRole::from_str(&row.get::<_, String>(3)?).unwrap(),
            personality_traits: row.get(4)?,
            physical_description: row.get(5)?,
            backstory: row.get(6)?,
            current_state: row.get(7)?,
            first_appearance_scene_id: None,
            created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    }).map_err(|e| {
        if e == rusqlite::Error::QueryReturnedNoRows {
            StoryError::not_found(format!("Character not found: {}", character_id))
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    Ok(json!({
        "characterId": character.id.to_string(),
        "projectId": character.story_project_id.to_string(),
        "name": character.name,
        "role": character.role.to_string(),
        "personalityTraits": character.personality_traits,
        "physicalDescription": character.physical_description,
        "backstory": character.backstory,
        "currentState": character.current_state
    }))
}

/// List all characters in a project
pub fn list_characters(conn: &Connection, params: Value) -> Result<Value> {
    let project_id_str = params
        .get("projectId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: projectId"))?;

    let project_id = Uuid::parse_str(project_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for projectId"))?;

    let mut stmt = conn.prepare(
        "SELECT id, name, role, personality_traits, current_state
         FROM characters
         WHERE story_project_id = ?1
         ORDER BY role, name"
    )?;

    let characters = stmt
        .query_map([project_id.to_string()], |row| {
            Ok(json!({
                "characterId": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "role": row.get::<_, String>(2)?,
                "personalityTraits": row.get::<_, Option<String>>(3)?,
                "currentState": row.get::<_, Option<String>>(4)?
            }))
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

    log::info!("Listed {} characters for project {}", characters.len(), project_id);

    Ok(json!({
        "characters": characters
    }))
}

/// Add a relationship between two characters
pub fn add_character_relationship(conn: &Connection, params: Value) -> Result<Value> {
    let source_id_str = params
        .get("sourceCharacterId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: sourceCharacterId"))?;

    let target_id_str = params
        .get("targetCharacterId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: targetCharacterId"))?;

    let source_id = Uuid::parse_str(source_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for sourceCharacterId"))?;

    let target_id = Uuid::parse_str(target_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for targetCharacterId"))?;

    if source_id == target_id {
        return Err(StoryError::validation("Cannot create relationship with self"));
    }

    let relationship_type_str = params
        .get("relationshipType")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: relationshipType"))?;

    let relationship_type = RelationshipType::from_str(relationship_type_str)
        .ok_or_else(|| StoryError::validation(format!("Invalid relationshipType: {}", relationship_type_str)))?;

    let description = params.get("description").and_then(|v| v.as_str());
    let strength = params.get("strength").and_then(|v| v.as_i64()).map(|n| n as i32);

    let relationship = CharacterRelationship {
        id: Uuid::new_v4(),
        source_character_id: source_id,
        target_character_id: target_id,
        relationship_type: relationship_type.clone(),
        description: description.map(|s| s.to_string()),
        strength,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    conn.execute(
        "INSERT INTO character_relationships (id, source_character_id, target_character_id, relationship_type, description, strength, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            relationship.id.to_string(),
            relationship.source_character_id.to_string(),
            relationship.target_character_id.to_string(),
            relationship.relationship_type.to_string(),
            &relationship.description,
            relationship.strength,
            relationship.created_at.to_rfc3339(),
            relationship.updated_at.to_rfc3339(),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            StoryError::duplicate("Relationship already exists between these characters")
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    log::info!("Created relationship: {} -> {}", source_id, target_id);

    Ok(json!({
        "relationshipId": relationship.id.to_string(),
        "relationshipType": relationship.relationship_type.to_string(),
        "description": relationship.description,
        "strength": relationship.strength
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::tools::project::create_story_project;
    use tempfile::tempdir;

    #[test]
    fn test_add_character_success() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        let params = json!({
            "projectId": project_id,
            "name": "Hero",
            "role": "protagonist",
            "personalityTraits": "Brave"
        });

        let result = add_character(&conn, params);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("characterId").is_some());
        assert_eq!(response.get("name").unwrap(), "Hero");
        assert_eq!(response.get("role").unwrap(), "protagonist");
    }

    #[test]
    fn test_list_characters() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        add_character(&conn, json!({"projectId": project_id, "name": "Hero", "role": "protagonist"})).unwrap();
        add_character(&conn, json!({"projectId": project_id, "name": "Villain", "role": "antagonist"})).unwrap();

        let result = list_characters(&conn, json!({"projectId": project_id}));
        assert!(result.is_ok());

        let response = result.unwrap();
        let chars = response.get("characters").unwrap().as_array().unwrap();
        assert_eq!(chars.len(), 2);
    }
}
