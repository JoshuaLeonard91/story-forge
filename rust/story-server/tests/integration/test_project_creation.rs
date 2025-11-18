use rusqlite::Connection;
use serde_json::json;
use story_server::db;
use tempfile::tempdir;

#[test]
fn test_create_story_project_contract() {
    // Setup: Create temporary database
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    // Test data matching MCP tool contract
    let params = json!({
        "title": "The Shadow Realm",
        "genre": "Epic Fantasy",
        "targetLength": "novel"
    });

    // This should fail initially as the tool is not yet implemented
    // The tool should create a project and return project details
    let result = create_story_project_tool(&conn, params);

    // Contract expectations:
    // - Returns projectId (UUID)
    // - Returns dbPath (string)
    // - Returns title, genre, intended_length
    // - Status should be "draft"
    // - Word count should be 0
    assert!(result.is_ok(), "createStoryProject tool should succeed");

    let response = result.unwrap();
    assert!(response.get("projectId").is_some(), "Should return projectId");
    assert!(response.get("title").is_some(), "Should return title");
    assert_eq!(response.get("title").unwrap(), "The Shadow Realm");
    assert_eq!(response.get("status").and_then(|v| v.as_str()), Some("draft"));
}

#[test]
fn test_create_story_project_duplicate_title() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    let params = json!({
        "title": "Unique Title",
        "genre": "Fantasy",
        "targetLength": "novel"
    });

    // Create first project
    let result1 = create_story_project_tool(&conn, params.clone());
    assert!(result1.is_ok());

    // Attempt to create duplicate
    let result2 = create_story_project_tool(&conn, params);
    assert!(result2.is_err(), "Should fail on duplicate title");
}

#[test]
fn test_create_story_project_validation() {
    let dir = tempdir().unwrap();
    let db_path = dir.path().join("test.db");
    let conn = db::initialize_database(&db_path).unwrap();

    // Test missing title
    let params = json!({
        "genre": "Fantasy",
        "targetLength": "novel"
    });

    let result = create_story_project_tool(&conn, params);
    assert!(result.is_err(), "Should fail without title");

    // Test invalid targetLength
    let params = json!({
        "title": "Test",
        "targetLength": "invalid_length"
    });

    let result = create_story_project_tool(&conn, params);
    assert!(result.is_err(), "Should fail with invalid targetLength");
}

// Placeholder function - will be replaced with actual tool implementation
fn create_story_project_tool(_conn: &Connection, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    // This will fail until we implement the actual tool in Phase 3
    Err("Tool not yet implemented".to_string())
}
