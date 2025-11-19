use anyhow::Result;
use rusqlite::Connection;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::types::Tool;

pub type ToolHandler = Arc<dyn Fn(&Connection, Value) -> Result<Value> + Send + Sync>;

pub struct ToolRegistry {
    tools: HashMap<String, ToolHandler>,
    tool_definitions: HashMap<String, Tool>,
    conn: Arc<Mutex<Connection>>,
}

impl ToolRegistry {
    pub fn new(conn: Connection) -> Self {
        ToolRegistry {
            tools: HashMap::new(),
            tool_definitions: HashMap::new(),
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn register<F>(&mut self, name: &str, description: &str, input_schema: Value, handler: F)
    where
        F: Fn(&Connection, Value) -> Result<Value> + Send + Sync + 'static,
    {
        self.tools.insert(name.to_string(), Arc::new(handler));
        self.tool_definitions.insert(
            name.to_string(),
            Tool {
                name: name.to_string(),
                description: description.to_string(),
                input_schema,
            },
        );
        log::info!("Registered tool: {}", name);
    }

    pub fn call_tool(&self, name: &str, params: Value) -> Result<Value> {
        let handler = self
            .tools
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;

        let conn = self.conn.lock().unwrap();
        handler(&conn, params)
    }

    pub fn list_tools(&self) -> Vec<Tool> {
        self.tool_definitions.values().cloned().collect()
    }

    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use serde_json::json;

    #[test]
    fn test_tool_registration() {
        let conn = Connection::open_in_memory().unwrap();
        let mut registry = ToolRegistry::new(conn);

        registry.register("test_tool", "A test tool", json!({"type": "object"}), |_conn, params| {
            Ok(json!({"received": params}))
        });

        assert!(registry.has_tool("test_tool"));
        assert_eq!(registry.list_tools().len(), 1);
    }

    #[test]
    fn test_tool_execution() {
        let conn = Connection::open_in_memory().unwrap();
        let mut registry = ToolRegistry::new(conn);

        registry.register("echo", "Echo tool", json!({"type": "object"}), |_conn, params| {
            Ok(params)
        });

        let result = registry.call_tool("echo", json!({"message": "hello"})).unwrap();
        assert_eq!(result, json!({"message": "hello"}));
    }

    #[test]
    fn test_tool_not_found() {
        let conn = Connection::open_in_memory().unwrap();
        let registry = ToolRegistry::new(conn);

        let result = registry.call_tool("nonexistent", json!({}));
        assert!(result.is_err());
    }
}
