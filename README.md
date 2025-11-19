# Story Forge

AI-assisted novel writing tool for Claude Code with intelligent project management, character tracking, and world building.

## Overview

Story Forge is a plugin for Claude Code that helps authors plan and write long-form fiction. It provides structured project management, character tracking, world building tools, and plot organization - all backed by a fast Rust MCP server with local SQLite storage.

Perfect for writers working on fantasy, science fiction, LitRPG, and progression fantasy novels.

## Features

**Story Project Management**
- Create projects with customizable plot structures
- Support for short stories, novellas, novels, and series
- Track project status and word counts

**Character Management**
- Detailed character profiles with personality, backstory, and physical descriptions
- Character relationships (ally, enemy, family, romantic, mentor, rival)
- Character state tracking throughout your story

**World Building**
- Define world rules with different scopes (universal, regional, situational)
- Keyword tagging for easy retrieval
- Organize magic systems, physics, laws, and customs

**Plot Structure**
- Multiple structure templates: Three-Act, Five-Act, Hero's Journey, Custom
- Hierarchical organization: Acts to Chapters to Scenes
- Automatic act generation based on selected structure

## Installation

### Quick Install (Recommended)

Install globally via npm:

```bash
npm install -g story-forge
```

Create a new story project:

```bash
story-forge init my-novel
cd my-novel
```

Open the folder in Claude Code and configure the MCP server:

```
/writer.mcp.setup
```

Follow the wizard to complete setup, then restart Claude Code to activate the MCP server.

### Manual Install

1. Clone this repository
2. Open the folder in Claude Code
3. The MCP server binary is pre-built in `bin/story-server.exe`

See [INSTALLATION.md](INSTALLATION.md) for detailed setup instructions.

## Quick Start

After installation, use these slash commands in Claude Code:

| Command | Description |
|---------|-------------|
| `/writer.mcp.setup` | Configure MCP server (one-time setup per project) |
| `/writer.start` | Create new story project with guided wizard |
| `/writer.projects` | List all story projects |
| `/writer.character.add` | Add new character with wizard |
| `/writer.world.rule` | Define world rule (magic system, physics, laws) |

### Example Workflow

1. Configure MCP server (first time only):
   ```
   /writer.mcp.setup
   ```
   Confirm setup and restart Claude Code

2. Create a new project:
   ```
   /writer.start
   ```
   Answer the wizard questions (title, genre, target length, plot structure)

3. Add your protagonist:
   ```
   /writer.character.add
   ```
   Provide name, role, personality traits, backstory

4. Define your magic system:
   ```
   /writer.world.rule
   ```
   Set name, description, scope, and examples

See [QUICKSTART.md](QUICKSTART.md) for a complete tutorial.

## Architecture

**Language:** Rust 1.75+ (memory safety, single binary distribution, fast performance)
**Database:** SQLite 3.40+ with FTS5 full-text search
**Protocol:** Model Context Protocol (MCP) over stdio
**Interface:** Markdown slash commands + Rust MCP server

```
.claude/commands/writer/    # Slash commands (user interface)
rust/story-server/          # Rust MCP server (data layer)
bin/story-server            # Compiled binary
data/*.db                   # SQLite databases (local storage)
.mcp.json                   # MCP server configuration
```

## Technical Details

### Database Schema

14 core entities for story management:
- Story projects (root container)
- Characters and character relationships
- World rules (magic systems, physics, customs)
- Plot structures (acts, chapters, scenes)
- Scene-character associations
- Character arcs and milestones
- Story summaries (hierarchical)
- Continuity alerts
- Progression systems (LitRPG/cultivation)

Full-text search indexes powered by SQLite FTS5 for:
- Character profiles
- World rules
- Scene content
- Story summaries

### MCP Tools

15 MCP tools available for programmatic access:

**Project Management:**
- `createStoryProject` - Create new story project
- `loadStoryProject` - Load existing project
- `listStoryProjects` - List all projects

**Character Management:**
- `addCharacter` - Add new character
- `getCharacter` - Retrieve character details
- `listCharacters` - List all characters
- `addCharacterRelationship` - Define character relationships

**World Building:**
- `addWorldRule` - Define world rule
- `getWorldRule` - Retrieve rule details
- `listWorldRules` - List all rules

**Plot Structure:**
- `initializePlotStructure` - Set up plot framework
- `addChapter` - Add chapter to structure
- `addScene` - Add scene to chapter
- `getPlotStructure` - Retrieve full plot hierarchy

### Development

To build from source:

```bash
cd rust/story-server
cargo build --release
```

Run tests:

```bash
cargo test
```

The plugin uses test-driven development with comprehensive test coverage.

## Performance

- Context retrieval: <500ms
- Database queries: single-digit milliseconds with FTS5 indexes
- Binary size: ~4.0 MB (single executable)
- Memory safe: Rust ownership guarantees

## Contributing

Contributions are welcome. Please:
1. Write tests for new features
2. Follow Rust style guidelines (run `cargo fmt`)
3. Ensure all tests pass before submitting

## License

MIT License - See [LICENSE](LICENSE) for details

## Support

- Report issues: [GitHub Issues](https://github.com/YOUR_USERNAME/story-forge/issues)
- Documentation: See [INSTALLATION.md](INSTALLATION.md) and [QUICKSTART.md](QUICKSTART.md)

---

Built with Rust and SQLite for Claude Code
