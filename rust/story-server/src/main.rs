use anyhow::Result;
use log::{error, info};
use std::env;
use std::path::PathBuf;
use story_server::{db, mcp, init_logging};

fn main() -> Result<()> {
    // Initialize logging
    init_logging();
    info!("Story Server MCP starting...");

    // Get data directory from environment or use default
    let data_dir = env::var("STORY_DATA_DIR")
        .unwrap_or_else(|_| "data".to_string());
    let data_path = PathBuf::from(data_dir);

    // Ensure data directory exists
    if !data_path.exists() {
        std::fs::create_dir_all(&data_path)?;
        info!("Created data directory: {:?}", data_path);
    }

    // Initialize default database (or could be done lazily per project)
    let db_path = data_path.join("story_server.db");
    let conn = db::initialize_database(&db_path)?;
    info!("Database initialized at {:?}", db_path);

    // Create tool registry and register tools
    let mut registry = mcp::ToolRegistry::new(conn);
    register_tools(&mut registry)?;

    // Create protocol handler
    let mut protocol = mcp::McpProtocolHandler::new();

    info!("Story Server ready - listening on stdin");

    // Main event loop: read requests from stdin, execute tools, write responses to stdout
    loop {
        match protocol.read_request() {
            Ok(request) => {
                let id = request.id.clone();
                let method = request.method.clone();

                // Handle MCP methods
                match method.as_str() {
                    "initialize" => {
                        // MCP initialization handshake
                        let result = serde_json::json!({
                            "protocolVersion": "2024-11-05",
                            "capabilities": {
                                "tools": {}
                            },
                            "serverInfo": {
                                "name": "story-db",
                                "version": env!("CARGO_PKG_VERSION")
                            }
                        });
                        if let Err(e) = protocol.send_success(id, result) {
                            error!("Failed to send initialize response: {}", e);
                        }
                    }
                    "tools/list" => {
                        // Return list of available tools
                        let tools = registry.list_tools();
                        let result = serde_json::json!({
                            "tools": tools
                        });
                        if let Err(e) = protocol.send_success(id, result) {
                            error!("Failed to send tools/list response: {}", e);
                        }
                    }
                    "tools/call" => {
                        // Call a tool
                        if let Some(params) = request.params {
                            if let Some(tool_name) = params.get("name").and_then(|v| v.as_str()) {
                                let tool_params = params.get("arguments").cloned().unwrap_or(serde_json::json!({}));


                                match registry.call_tool(tool_name, tool_params) {
                                    Ok(result) => {
                                        // Wrap result in MCP content format
                                        let wrapped = serde_json::json!({
                                            "content": [
                                                {
                                                    "type": "text",
                                                    "text": serde_json::to_string(&result).unwrap_or_else(|_| result.to_string())
                                                }
                                            ]
                                        });
                                        if let Err(e) = protocol.send_success(id, wrapped) {
                                            error!("Failed to send success response: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Tool execution error: {}", e);
                                        if let Err(e) = protocol.send_error(
                                            id,
                                            mcp::types::INTERNAL_ERROR,
                                            e.to_string(),
                                        ) {
                                            error!("Failed to send error response: {}", e);
                                        }
                                    }
                                }
                            } else {
                                if let Err(e) = protocol.send_error(
                                    id,
                                    mcp::types::INVALID_PARAMS,
                                    "Missing 'name' parameter".to_string(),
                                ) {
                                    error!("Failed to send error response: {}", e);
                                }
                            }
                        } else {
                            if let Err(e) = protocol.send_error(
                                id,
                                mcp::types::INVALID_PARAMS,
                                "Missing parameters".to_string(),
                            ) {
                                error!("Failed to send error response: {}", e);
                            }
                        }
                    }
                    _ => {
                        if let Err(e) = protocol.send_error(
                            id,
                            mcp::types::METHOD_NOT_FOUND,
                            format!("Unknown method: {}", method),
                        ) {
                            error!("Failed to send error response: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                // Check if it's EOF (stdin closed)
                if e.to_string().contains("No input from stdin") {
                    info!("stdin closed - shutting down gracefully");
                    break;
                }
                error!("Failed to read request: {}", e);
                // Continue reading despite errors
            }
        }
    }

    info!("Story Server MCP shutting down");
    Ok(())
}

fn register_tools(registry: &mut mcp::ToolRegistry) -> Result<()> {
    use story_server::tools;
    use serde_json::json;

    // Project management tools
    registry.register(
        "mcp__story-db__createStoryProject",
        "Create a new story project with title, genre, and plot structure",
        json!({"type": "object", "properties": {"title": {"type": "string"}, "genre": {"type": "string"}, "targetLength": {"type": "string"}, "plotStructure": {"type": "string"}}, "required": ["title"]}),
        |conn, params| {
            tools::create_story_project(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__loadStoryProject",
        "Load an existing story project by ID",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}}, "required": ["projectId"]}),
        |conn, params| {
            tools::load_story_project(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__listStoryProjects",
        "List all story projects in the database",
        json!({"type": "object"}),
        |conn, params| {
            tools::list_story_projects(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    // Character management tools
    registry.register(
        "mcp__story-db__addCharacter",
        "Add a new character to a story project",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}, "name": {"type": "string"}, "role": {"type": "string"}}, "required": ["projectId", "name"]}),
        |conn, params| {
            tools::add_character(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__getCharacter",
        "Get a character by ID",
        json!({"type": "object", "properties": {"characterId": {"type": "string"}}, "required": ["characterId"]}),
        |conn, params| {
            tools::get_character(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__listCharacters",
        "List all characters in a story project",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}}, "required": ["projectId"]}),
        |conn, params| {
            tools::list_characters(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__addCharacterRelationship",
        "Add a relationship between two characters",
        json!({"type": "object", "properties": {"character1Id": {"type": "string"}, "character2Id": {"type": "string"}, "relationshipType": {"type": "string"}}, "required": ["character1Id", "character2Id", "relationshipType"]}),
        |conn, params| {
            tools::add_character_relationship(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    // World building tools
    registry.register(
        "mcp__story-db__addWorldRule",
        "Add a world-building rule (magic system, technology, social structure, etc.)",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}, "ruleName": {"type": "string"}, "description": {"type": "string"}}, "required": ["projectId", "ruleName", "description"]}),
        |conn, params| {
            tools::add_world_rule(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__getWorldRule",
        "Get a world rule by ID",
        json!({"type": "object", "properties": {"ruleId": {"type": "string"}}, "required": ["ruleId"]}),
        |conn, params| {
            tools::get_world_rule(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__listWorldRules",
        "List all world rules for a story project",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}}, "required": ["projectId"]}),
        |conn, params| {
            tools::list_world_rules(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    // Plot structure tools
    registry.register(
        "mcp__story-db__initializePlotStructure",
        "Initialize plot structure for a story (three-act, five-act, hero's journey)",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}, "structureType": {"type": "string"}}, "required": ["projectId", "structureType"]}),
        |conn, params| {
            tools::initialize_plot_structure(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__addChapter",
        "Add a chapter to the story",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}, "chapterNumber": {"type": "number"}, "title": {"type": "string"}}, "required": ["projectId", "chapterNumber"]}),
        |conn, params| {
            tools::add_chapter(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__addScene",
        "Add a scene to a chapter",
        json!({"type": "object", "properties": {"chapterId": {"type": "string"}, "sceneNumber": {"type": "number"}, "content": {"type": "string"}}, "required": ["chapterId", "sceneNumber"]}),
        |conn, params| {
            tools::add_scene(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    registry.register(
        "mcp__story-db__getPlotStructure",
        "Get the plot structure for a story project",
        json!({"type": "object", "properties": {"projectId": {"type": "string"}}, "required": ["projectId"]}),
        |conn, params| {
            tools::get_plot_structure(conn, params)
                .map_err(|e| anyhow::anyhow!(e.to_string()))
        },
    );

    info!("Registered {} MCP tools", registry.list_tools().len());
    Ok(())
}
