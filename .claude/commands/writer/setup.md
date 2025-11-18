---
description: Set up Story Forge MCP server for this project
---

You are helping the user set up the Story Forge MCP server for their writing project. This is a one-time setup process.

## Setup Process

Follow these steps:

1. **Check if already set up** - Look for `.mcp.json` in the current directory
   - If it exists and has `story-db` configured, tell them it's already set up
   - If not, continue with setup

2. **Explain what you're setting up:**
   "I'll configure the Story Forge MCP server which provides the database backend for your writing project. This involves:
   - Creating a `.mcp.json` configuration file
   - Setting up the `story-db` MCP server entry
   - Configuring the data directory path"

3. **Ask for confirmation:**
   "Would you like me to configure the MCP server now? (yes/no)"

4. **If yes, create the configuration:**
   Create a `.mcp.json` file in the current directory with this content:

```json
{
  "mcpServers": {
    "story-db": {
      "command": "story-server",
      "args": [],
      "env": {
        "RUST_LOG": "info",
        "STORY_DATA_DIR": "data"
      }
    }
  }
}
```

5. **Create data directory:**
   - Create a `data/` folder in the current directory if it doesn't exist
   - This is where SQLite databases will be stored

6. **Check for binary:**
   - Check if `story-server.exe` (Windows) or `story-server` (Unix) exists in the system PATH
   - If NOT found, provide instructions:
     ```
     The story-server binary is not installed. You have two options:

     Option 1: Download pre-built binary
     - Windows: Download from GitHub releases
     - macOS/Linux: Download from GitHub releases
     - Place in a directory in your PATH

     Option 2: Build from source
     - Clone the repository
     - Run: cd rust/story-server && cargo build --release
     - Copy the binary to your PATH

     Repository: https://github.com/YOUR_USERNAME/story-forge
     ```

7. **Confirm setup complete:**
   "MCP server configured successfully! Next steps:
   - Restart Claude Code to load the MCP server
   - Try `/writer.start` to create your first story project
   - The story data will be saved in the `data/` directory"

IMPORTANT: Always ask for user permission before creating files. Explain clearly what you're doing at each step.
