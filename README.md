# Story Writing Engine - Claude Code Plugin

**Version**: 0.1.0 (MVP)
**Status**: ✅ Phase 1-3 Complete (58/162 tasks completed)

A production-grade AI-assisted novel writing plugin for Claude Code with selective memory management, continuity tracking, and optional progression systems (LitRPG/cultivation).

## Features (MVP - User Story 1)

✅ **Story Project Management**
- Create new story projects with customizable plot structures
- Support for short stories, novellas, novels, and series
- Track project status and word counts

✅ **Character Management**
- Add characters with detailed attributes (role, personality, backstory, physical description)
- Define character relationships (ally, enemy, family, romantic, mentor, rival)
- Track character state throughout the story

✅ **World Building**
- Define world rules with scopes (universal, regional, situational)
- Keyword tagging for context retrieval
- Examples and detailed descriptions

✅ **Plot Structure**
- Multiple structure types: Three-Act, Five-Act, Hero's Journey, Custom
- Hierarchical organization: Acts → Chapters → Scenes
- Automatic act generation based on selected structure

## Architecture

**Language**: Rust 1.75+ (memory safety, single binary distribution, <500ms performance)
**Database**: SQLite 3.40+ with FTS5 (full-text search)
**Protocol**: Model Context Protocol (MCP) - JSON-RPC over stdio
**Interface**: Markdown slash commands + Rust MCP server

```
.claude/commands/writer/    # Slash commands (user interface)
rust/story-server/          # Rust MCP server (data layer)
bin/story-server.exe        # Compiled binary (4.0 MB)
data/*.db                   # SQLite databases (one per book)
.mcp.json                   # MCP server configuration
```

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

Open the folder in Claude Code and you're ready to write!

### Manual Install (Development)

1. Clone this repository
2. Binary is pre-built in `bin/story-server.exe`
3. MCP configuration is in `.mcp.json`
4. Slash commands are in `.claude/commands/writer/`

See [INSTALLATION.md](INSTALLATION.md) for detailed setup instructions.

### Available Commands

| Command | Description |
|---------|-------------|
| `/writer.start` | Create new story project with guided wizard |
| `/writer.projects` | List all story projects |
| `/writer.character.add` | Add new character with wizard |
| `/writer.world.rule` | Define world rule (magic system, physics, laws) |

## Usage Example

```bash
# 1. Start new project
/writer.start

# Answer wizard questions:
# - Title: "The Shadow Realm"
# - Genre: Epic Fantasy
# - Target Length: Novel
# - Plot Structure: Hero's Journey

# 2. Add protagonist
/writer.character.add

# Provide details:
# - Name: "Kael Darkblade"
# - Role: Protagonist
# - Personality: "Determined, strategic, haunted by past"
# - Backstory: "Former knight seeking redemption..."

# 3. Define magic system
/writer.world.rule

# Define rule:
# - Name: "Magic System: Shadow Binding"
# - Description: "Casters bind shadows to their will..."
# - Scope: Universal
```

## Technical Details

### Database Schema

**14 Core Entities:**
- `story_projects` - Root container for all story elements
- `characters` - Character profiles with attributes
- `character_relationships` - Directed graph of relationships
- `world_rules` - Immutable laws governing the story world
- `plot_structures` - Hierarchical plot organization
- `acts` - Major structural divisions
- `chapters` - Chapter-level organization
- `scenes` - Atomic units of story content
- `scene_characters` - Many-to-many scene/character association
- `character_arcs` - Character transformation journeys
- `arc_milestones` - Key checkpoints in arcs
- `story_summaries` - Hierarchical summaries
- `continuity_alerts` - Detected contradictions/errors
- `progression_systems` - Optional LitRPG/cultivation mechanics

**FTS5 Indexes** for full-text search:
- `characters_fts` - Search character profiles
- `world_rules_fts` - Search world rules
- `scenes_fts` - Search scene content
- `summaries_fts` - Search summaries

### MCP Tools (15 implemented)

**Project Management:**
- `mcp__story-db__createStoryProject`
- `mcp__story-db__loadStoryProject`
- `mcp__story-db__listStoryProjects`

**Character Management:**
- `mcp__story-db__addCharacter`
- `mcp__story-db__getCharacter`
- `mcp__story-db__listCharacters`
- `mcp__story-db__addCharacterRelationship`

**World Building:**
- `mcp__story-db__addWorldRule`
- `mcp__story-db__getWorldRule`
- `mcp__story-db__listWorldRules`

**Plot Structure:**
- `mcp__story-db__initializePlotStructure`
- `mcp__story-db__addChapter`
- `mcp__story-db__addScene`
- `mcp__story-db__getPlotStructure`

### Test Coverage

**33 tests passing** (100% success rate):
- Unit tests for all models and utilities
- Integration tests for database operations
- Contract tests for MCP tools
- End-to-end workflow tests

```bash
# Run tests
cd rust/story-server
cargo test

# Build release binary
cargo build --release
```

## Roadmap

### ✅ Phase 1: Setup (Complete)
- [X] Project structure
- [X] Rust dependencies
- [X] MCP configuration

### ✅ Phase 2: Foundational (Complete)
- [X] Database schema & migrations
- [X] MCP protocol infrastructure
- [X] Core models & error handling

### ✅ Phase 3: User Story 1 - MVP (Complete)
- [X] Story project management
- [X] Character management with relationships
- [X] World building with rules
- [X] Plot structure with acts/chapters/scenes
- [X] 15 MCP tools implemented
- [X] 4 slash commands created
- [X] All tests passing

### 🔄 Phase 4: User Story 2 - Context-Aware Writing (Next)
- [ ] Selective memory/context retrieval (<5000 words)
- [ ] Relevance scoring algorithm
- [ ] Scene writing assistance
- [ ] FTS5 search integration

### 📋 Phase 5: User Story 3 - Character Arcs
- [ ] Arc tracking with milestones
- [ ] Progress monitoring
- [ ] Arc state in context

### 📋 Phase 6: User Story 4 - Summaries
- [ ] Auto-generated summaries (scene/chapter/act/overall)
- [ ] Hierarchical aggregation

### 📋 Phase 7: User Story 5 - Continuity Validation
- [ ] Tier 1: State-based tracking (offline, ~60% detection)
- [ ] Tier 2: AI-powered analysis (optional, ~95% detection)
- [ ] Alert system for contradictions

### 📋 Phase 8-10: Optional Features & Polish
- [ ] Progression systems (LitRPG, cultivation templates)
- [ ] Export to text/markdown
- [ ] Statistics dashboard
- [ ] Performance optimization
- [ ] Documentation

## Development

### Project Structure

```
E:\BookPlugin/
├── .claude/
│   └── commands/writer/        # Slash commands
│       ├── start.md
│       ├── projects.md
│       ├── character/
│       │   └── add.md
│       └── world/
│           └── rule.md
├── rust/story-server/          # Rust MCP server
│   ├── src/
│   │   ├── db/                 # Database layer
│   │   ├── mcp/                # MCP protocol
│   │   ├── models/             # Data models
│   │   ├── tools/              # MCP tool implementations
│   │   ├── context/            # Context retrieval (Phase 4)
│   │   ├── continuity/         # Continuity checking (Phase 7)
│   │   └── systems/            # Progression systems (Phase 8)
│   ├── tests/                  # Test suite
│   └── Cargo.toml
├── bin/
│   └── story-server.exe        # Compiled binary (4.0 MB)
├── data/                       # SQLite databases
├── specs/                      # Planning documents
│   └── 001-story-writing-engine/
│       ├── spec.md
│       ├── plan.md
│       ├── tasks.md
│       ├── data-model.md
│       └── contracts/
└── .mcp.json                   # MCP server configuration
```

### Contributing

This plugin follows TDD (Test-Driven Development):
1. Write tests first (verify they fail)
2. Implement features
3. Verify tests pass
4. Refactor

### Constitution Principles

1. **Plugin-First Architecture** - Clear boundaries, defined lifecycle
2. **Test-First Development** - NON-NEGOTIABLE: Red-Green-Refactor cycle
3. **Plugin Interface Contracts** - Semantic versioning, documented APIs
4. **Observable Plugin Behavior** - Structured logging, performance metrics
5. **Simplicity and YAGNI** - MVP-first, defer complexity

## Performance

- **Context Retrieval**: <500ms target (Rust performance)
- **Database Queries**: Single-digit milliseconds with FTS5 indexes
- **Binary Size**: 4.0 MB (single executable)
- **Memory Safety**: Guaranteed by Rust's ownership system

## License

MIT

## Credits

- **Architecture**: Hybrid Claude Code plugin (Markdown + Rust MCP server)
- **Database**: SQLite with FTS5
- **Protocol**: Model Context Protocol (MCP)
- **Language**: Rust 1.75+

---

**Status**: MVP Complete ✅
**Next Phase**: User Story 2 (Context-Aware Scene Writing)
**Progress**: 58/162 tasks (35% complete, MVP functional)
