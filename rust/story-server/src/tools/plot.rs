use crate::error::{Result, StoryError};
use crate::models::{PlotStructure, StructureType};
use chrono::Utc;
use rusqlite::Connection;
use serde_json::{json, Value};
use uuid::Uuid;

/// Initialize plot structure for a story project
pub fn initialize_plot_structure(conn: &Connection, params: Value) -> Result<Value> {
    let project_id_str = params
        .get("projectId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: projectId"))?;

    let project_id = Uuid::parse_str(project_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for projectId"))?;

    let structure_type_str = params
        .get("structureType")
        .and_then(|v| v.as_str())
        .unwrap_or("three_act");

    let structure_type = StructureType::from_str(structure_type_str)
        .ok_or_else(|| StoryError::validation(format!("Invalid structureType: {}", structure_type_str)))?;

    let plot_structure = PlotStructure {
        id: Uuid::new_v4(),
        story_project_id: project_id,
        structure_type: structure_type.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Insert plot structure
    conn.execute(
        "INSERT INTO plot_structures (id, story_project_id, structure_type, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            plot_structure.id.to_string(),
            plot_structure.story_project_id.to_string(),
            plot_structure.structure_type.to_string(),
            plot_structure.created_at.to_rfc3339(),
            plot_structure.updated_at.to_rfc3339(),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            StoryError::duplicate("Plot structure already exists for this project")
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    // Create default acts based on structure type
    let acts = match structure_type {
        StructureType::ThreeAct => vec![
            ("Act 1: Setup", 1),
            ("Act 2: Confrontation", 2),
            ("Act 3: Resolution", 3),
        ],
        StructureType::FiveAct => vec![
            ("Act 1: Exposition", 1),
            ("Act 2: Rising Action", 2),
            ("Act 3: Climax", 3),
            ("Act 4: Falling Action", 4),
            ("Act 5: Resolution", 5),
        ],
        StructureType::HeroJourney => vec![
            ("Part 1: Ordinary World", 1),
            ("Part 2: Call to Adventure", 2),
            ("Part 3: Tests & Trials", 3),
            ("Part 4: Ordeal & Reward", 4),
            ("Part 5: Return & Transformation", 5),
        ],
        StructureType::Custom => vec![],
    };

    let mut act_ids = Vec::new();
    for (name, position) in acts {
        let act_id = Uuid::new_v4();
        conn.execute(
            "INSERT INTO acts (id, plot_structure_id, name, position, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                act_id.to_string(),
                plot_structure.id.to_string(),
                name,
                position,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
            ),
        )?;
        act_ids.push(json!({
            "actId": act_id.to_string(),
            "name": name,
            "position": position
        }));
    }

    log::info!("Created plot structure: {} with {} acts", plot_structure.id, act_ids.len());

    Ok(json!({
        "plotStructureId": plot_structure.id.to_string(),
        "structureType": plot_structure.structure_type.to_string(),
        "acts": act_ids,
        "createdAt": plot_structure.created_at.to_rfc3339()
    }))
}

/// Add a chapter to an act
pub fn add_chapter(conn: &Connection, params: Value) -> Result<Value> {
    let act_id_str = params
        .get("actId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: actId"))?;

    let act_id = Uuid::parse_str(act_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for actId"))?;

    let title = params.get("title").and_then(|v| v.as_str());
    let number = params
        .get("number")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| StoryError::validation("Missing required field: number"))?;

    // Get current max position for global chapter ordering
    let position: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM chapters",
            [],
            |row| row.get(0),
        )
        .unwrap_or(1);

    let chapter_id = Uuid::new_v4();

    conn.execute(
        "INSERT INTO chapters (id, act_id, title, number, position, status, word_count, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            chapter_id.to_string(),
            act_id.to_string(),
            title,
            number as i32,
            position,
            "planned",
            0,
            Utc::now().to_rfc3339(),
            Utc::now().to_rfc3339(),
        ),
    ).map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            StoryError::duplicate(format!("Chapter {} already exists in this act", number))
        } else {
            StoryError::DatabaseError(e)
        }
    })?;

    log::info!("Created chapter: {} (number {})", chapter_id, number);

    Ok(json!({
        "chapterId": chapter_id.to_string(),
        "title": title,
        "number": number,
        "position": position,
        "status": "planned",
        "wordCount": 0
    }))
}

/// Add a scene to a chapter
pub fn add_scene(conn: &Connection, params: Value) -> Result<Value> {
    let chapter_id_str = params
        .get("chapterId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: chapterId"))?;

    let chapter_id = Uuid::parse_str(chapter_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for chapterId"))?;

    let title = params.get("title").and_then(|v| v.as_str());
    let location = params.get("location").and_then(|v| v.as_str());
    let time_description = params.get("timeDescription").and_then(|v| v.as_str());
    let scene_outline = params.get("sceneOutline").and_then(|v| v.as_str());

    // Get current max position in chapter
    let position: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(position), 0) + 1 FROM scenes WHERE chapter_id = ?1",
            [chapter_id.to_string()],
            |row| row.get(0),
        )
        .unwrap_or(1);

    let scene_id = Uuid::new_v4();

    conn.execute(
        "INSERT INTO scenes (id, chapter_id, title, position, location, time_description, content, word_count, status, scene_outline, ai_generated, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        (
            scene_id.to_string(),
            chapter_id.to_string(),
            title,
            position,
            location,
            time_description,
            "", // Empty content initially
            0,
            "planned",
            scene_outline,
            0, // false
            Utc::now().to_rfc3339(),
            Utc::now().to_rfc3339(),
        ),
    )?;

    log::info!("Created scene: {} (position {})", scene_id, position);

    Ok(json!({
        "sceneId": scene_id.to_string(),
        "title": title,
        "position": position,
        "location": location,
        "timeDescription": time_description,
        "sceneOutline": scene_outline,
        "status": "planned",
        "wordCount": 0
    }))
}

/// Get complete plot structure for a project
pub fn get_plot_structure(conn: &Connection, params: Value) -> Result<Value> {
    let project_id_str = params
        .get("projectId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StoryError::validation("Missing required field: projectId"))?;

    let project_id = Uuid::parse_str(project_id_str)
        .map_err(|_| StoryError::validation("Invalid UUID format for projectId"))?;

    // Get plot structure
    let mut stmt = conn.prepare(
        "SELECT id, structure_type FROM plot_structures WHERE story_project_id = ?1"
    )?;

    let (plot_id, structure_type): (String, String) = stmt
        .query_row([project_id.to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .map_err(|e| {
            if e == rusqlite::Error::QueryReturnedNoRows {
                StoryError::not_found("Plot structure not found for this project")
            } else {
                StoryError::DatabaseError(e)
            }
        })?;

    // Get acts with their chapters and scenes
    let mut stmt = conn.prepare(
        "SELECT id, name, position, description FROM acts WHERE plot_structure_id = ?1 ORDER BY position"
    )?;

    let mut acts = Vec::new();
    let act_rows = stmt.query_map([&plot_id], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    })?;

    for act_row in act_rows {
        let (act_id, name, position, description) = act_row?;

        // Get chapters for this act
        let mut chapter_stmt = conn.prepare(
            "SELECT id, title, number, status, word_count FROM chapters WHERE act_id = ?1 ORDER BY number"
        )?;

        let chapters = chapter_stmt
            .query_map([&act_id], |row| {
                Ok(json!({
                    "chapterId": row.get::<_, String>(0)?,
                    "title": row.get::<_, Option<String>>(1)?,
                    "number": row.get::<_, i32>(2)?,
                    "status": row.get::<_, String>(3)?,
                    "wordCount": row.get::<_, i32>(4)?
                }))
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        acts.push(json!({
            "actId": act_id,
            "name": name,
            "position": position,
            "description": description,
            "chapters": chapters
        }));
    }

    Ok(json!({
        "plotStructureId": plot_id,
        "structureType": structure_type,
        "acts": acts
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::tools::project::create_story_project;
    use tempfile::tempdir;

    #[test]
    fn test_initialize_plot_structure_three_act() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        let params = json!({
            "projectId": project_id,
            "structureType": "three_act"
        });

        let result = initialize_plot_structure(&conn, params);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("plotStructureId").is_some());
        assert_eq!(response.get("structureType").unwrap(), "three_act");

        let acts = response.get("acts").unwrap().as_array().unwrap();
        assert_eq!(acts.len(), 3);
    }

    #[test]
    fn test_add_chapter_and_scene() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        let plot = initialize_plot_structure(&conn, json!({"projectId": project_id})).unwrap();
        let acts = plot.get("acts").unwrap().as_array().unwrap();
        let act_id = acts[0].get("actId").unwrap().as_str().unwrap();

        // Add chapter
        let chapter_result = add_chapter(&conn, json!({"actId": act_id, "number": 1, "title": "Chapter One"}));
        assert!(chapter_result.is_ok());

        let chapter = chapter_result.unwrap();
        let chapter_id = chapter.get("chapterId").unwrap().as_str().unwrap();

        // Add scene
        let scene_result = add_scene(&conn, json!({"chapterId": chapter_id, "title": "Opening Scene"}));
        assert!(scene_result.is_ok());

        let scene = scene_result.unwrap();
        assert!(scene.get("sceneId").is_some());
        assert_eq!(scene.get("position").unwrap(), 1);
    }

    #[test]
    fn test_get_plot_structure() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = db::initialize_database(&db_path).unwrap();

        let project = create_story_project(&conn, json!({"title": "Test", "targetLength": "novel"})).unwrap();
        let project_id = project.get("projectId").unwrap().as_str().unwrap();

        initialize_plot_structure(&conn, json!({"projectId": project_id})).unwrap();

        let result = get_plot_structure(&conn, json!({"projectId": project_id}));
        assert!(result.is_ok());

        let structure = result.unwrap();
        assert!(structure.get("plotStructureId").is_some());
        assert!(structure.get("acts").unwrap().as_array().is_some());
    }
}
