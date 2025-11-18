use rusqlite::Connection;
use serde_json::json;
use story_server::db;
use tempfile::tempdir;
use uuid::Uuid;

#[test]
fn test_add_character_contract() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    // Assume we have a project ID
    let project_id = Uuid::new_v4();

    let params = json!({
        "projectId": project_id.to_string(),
        "name": "Jin Woo",
        "role": "protagonist",
        "personalityTraits": "Determined, strategic, protective",
        "physicalDescription": "Black hair, lean build, piercing eyes",
        "backstory": "Former E-rank hunter who gained mysterious powers"
    });

    let result = add_character_tool(&conn, params);

    // Contract expectations:
    // - Returns characterId (UUID)
    // - Returns name, role, and all provided attributes
    // - Character is stored in database
    assert!(result.is_ok(), "addCharacter tool should succeed");

    let response = result.unwrap();
    assert!(response.get("characterId").is_some(), "Should return characterId");
    assert_eq!(response.get("name").and_then(|v| v.as_str()), Some("Jin Woo"));
    assert_eq!(response.get("role").and_then(|v| v.as_str()), Some("protagonist"));
}

#[test]
fn test_add_character_duplicate_name() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    let project_id = Uuid::new_v4();

    let params = json!({
        "projectId": project_id.to_string(),
        "name": "Same Name",
        "role": "protagonist"
    });

    // Add first character
    let result1 = add_character_tool(&conn, params.clone());
    assert!(result1.is_ok());

    // Attempt duplicate
    let result2 = add_character_tool(&conn, params);
    assert!(result2.is_err(), "Should fail on duplicate character name in same project");
}

#[test]
fn test_add_character_validation() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    // Missing required fields
    let params = json!({
        "projectId": Uuid::new_v4().to_string(),
        "role": "protagonist"
    });

    let result = add_character_tool(&conn, params);
    assert!(result.is_err(), "Should fail without name");

    // Invalid role
    let params = json!({
        "projectId": Uuid::new_v4().to_string(),
        "name": "Test",
        "role": "invalid_role"
    });

    let result = add_character_tool(&conn, params);
    assert!(result.is_err(), "Should fail with invalid role");
}

// Placeholder function
fn add_character_tool(_conn: &Connection, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    Err("Tool not yet implemented".to_string())
}
