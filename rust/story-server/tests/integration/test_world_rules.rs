use rusqlite::Connection;
use serde_json::json;
use story_server::db;
use tempfile::tempdir;
use uuid::Uuid;

#[test]
fn test_add_world_rule_contract() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    let project_id = Uuid::new_v4();

    let params = json!({
        "projectId": project_id.to_string(),
        "name": "Magic System: Mana Cost",
        "description": "All magic requires mana. Stronger spells consume more mana.",
        "scope": "universal",
        "examples": "Fireball costs 50 mana, Healing costs 30 mana",
        "keywords": ["magic", "mana", "spell", "cost"]
    });

    let result = add_world_rule_tool(&conn, params);

    // Contract expectations:
    // - Returns ruleId (UUID)
    // - Returns name, description, scope
    // - Keywords stored as JSON array
    assert!(result.is_ok(), "addWorldRule tool should succeed");

    let response = result.unwrap();
    assert!(response.get("ruleId").is_some(), "Should return ruleId");
    assert_eq!(response.get("name").and_then(|v| v.as_str()), Some("Magic System: Mana Cost"));
    assert_eq!(response.get("scope").and_then(|v| v.as_str()), Some("universal"));
}

#[test]
fn test_add_world_rule_duplicate_name() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    let project_id = Uuid::new_v4();

    let params = json!({
        "projectId": project_id.to_string(),
        "name": "Unique Rule",
        "description": "Test rule",
        "scope": "universal"
    });

    let result1 = add_world_rule_tool(&conn, params.clone());
    assert!(result1.is_ok());

    let result2 = add_world_rule_tool(&conn, params);
    assert!(result2.is_err(), "Should fail on duplicate rule name");
}

#[test]
fn test_add_world_rule_validation() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    // Missing required fields
    let params = json!({
        "projectId": Uuid::new_v4().to_string(),
        "name": "Test Rule"
    });

    let result = add_world_rule_tool(&conn, params);
    assert!(result.is_err(), "Should fail without description");

    // Invalid scope
    let params = json!({
        "projectId": Uuid::new_v4().to_string(),
        "name": "Test",
        "description": "Test",
        "scope": "invalid_scope"
    });

    let result = add_world_rule_tool(&conn, params);
    assert!(result.is_err(), "Should fail with invalid scope");
}

// Placeholder function
fn add_world_rule_tool(_conn: &Connection, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    Err("Tool not yet implemented".to_string())
}
