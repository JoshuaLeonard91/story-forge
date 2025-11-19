---
description: Set up Story Forge MCP server for this project
---

You are helping the user set up the Story Forge MCP server for their writing project. This is a one-time setup process per project.

## Setup Process

Follow these steps:

1. **Check if already set up** - Look for `.mcp.json` in the current directory
   - If it exists and has `story-db` configured, tell them: "MCP server is already configured for this project."
   - If not, continue with setup

2. **Check for story-server binary:**
   Check if the binary is available by running: `which story-server`

   If NOT found (command fails or returns empty), provide installation instructions:
   ```
   The story-server binary is not installed globally. Please install it first:

   npm install -g story-forge

   After installation, run /writer.mcp.setup again.
   ```
   Stop here if binary is not found.

3. **Explain what you're setting up:**
   "I'll configure the Story Forge MCP server for this project. This involves:
   - Creating a `.mcp.json` configuration file
   - Setting up the `story-db` MCP server entry
   - Creating a `data/` directory for SQLite databases"

4. **Ask for confirmation using AskUserQuestion:**
   Use the AskUserQuestion tool with this configuration:
   - Question: "Would you like me to configure the MCP server now?"
   - Header: "Setup MCP"
   - Options:
     - "Yes" - "Configure the MCP server and create necessary files"
     - "No" - "Skip setup for now"
   - multiSelect: false

5. **If they select "Yes", create the configuration:**

   Create `.mcp.json` in the current directory.

   **IMPORTANT:** On Windows, use the `cmd` wrapper format. Detect the platform first.

   **For Windows:**
   ```json
   {
     "mcpServers": {
       "story-db": {
         "command": "cmd",
         "args": ["/c", "story-server"],
         "env": {
           "RUST_LOG": "info",
           "STORY_DATA_DIR": "data"
         }
       }
     }
   }
   ```

   **For macOS/Linux:**
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

   Use the Write tool to create this file.

6. **Create data directory:**

   Use the Write tool to create the data directory structure:
   - Create an empty `.gitkeep` file at `data/.gitkeep`

   DO NOT use Bash commands - use the Write tool directly to ensure cross-platform compatibility.

   This is where SQLite databases will be stored. The MCP server will automatically create `data/story_server.db` when you first use the tools.

7. **Confirm setup complete:**
   "MCP server configured successfully!

   Next steps:
   - Restart Claude Code to load the MCP server
   - Try /writer.start to create your first story project
   - Story data will be saved in the data/ directory

   Available commands:
   - /writer.start - Create new story project
   - /writer.projects - List all your projects
   - /writer.character.add - Add characters
   - /writer.world.rule - Define world rules"

8. **If they select "No":**
   "Setup cancelled. You can run /writer.mcp.setup again when you're ready to configure the MCP server."

IMPORTANT:
- Always check for binary availability FIRST
- Use AskUserQuestion tool for yes/no confirmation
- Only create files after explicit confirmation
- Explain clearly what you're doing at each step
- Follow the official MCP docs format exactly
