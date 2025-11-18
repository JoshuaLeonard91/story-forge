# Installation Guide - Story Writing Engine Plugin

## Quick Install (Recommended)

### Option 1: Download Pre-built Binary (Easiest)

1. **Download the plugin**
   ```bash
   # Clone from GitHub (or download ZIP)
   git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
   cd story-writing-engine
   ```

2. **The binary is already included** - No compilation needed!
   - Pre-built binary is in `bin/story-server.exe` (Windows)
   - For macOS/Linux, see "Build from Source" below

3. **Install in Claude Code**
   - Copy the entire `story-writing-engine` folder to your projects directory
   - Claude Code will automatically detect the `.mcp.json` configuration
   - Restart Claude Code

4. **Verify installation**
   ```bash
   # In Claude Code, type:
   /writer.start

   # You should see the project creation wizard
   ```

---

## Option 2: Install via Git (For Developers)

```bash
# 1. Clone the repository
git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
cd story-writing-engine

# 2. Build from source (requires Rust 1.75+)
cd rust/story-server
cargo build --release

# 3. Copy binary to bin/
cp target/release/story-server ../../../bin/story-server

# 4. Return to project root
cd ../..

# 5. Open in Claude Code
# Claude Code will auto-detect the plugin
```

---

## Option 3: Build from Source (All Platforms)

### Prerequisites

- **Rust 1.75+** - Install from https://rustup.rs/
- **Claude Code** - Latest version

### Windows

```bash
# 1. Install Rust (if not already installed)
# Download from https://rustup.rs/ and run the installer

# 2. Clone repository
git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
cd story-writing-engine

# 3. Build the MCP server
cd rust/story-server
cargo build --release

# 4. Copy binary
copy target\release\story-server.exe ..\..\bin\story-server.exe

# 5. Done! Open the folder in Claude Code
```

### macOS / Linux

```bash
# 1. Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Clone repository
git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
cd story-writing-engine

# 3. Build the MCP server
cd rust/story-server
cargo build --release

# 4. Copy binary
cp target/release/story-server ../../bin/story-server
chmod +x ../../bin/story-server

# 5. Done! Open the folder in Claude Code
```

---

## Verify Installation

After installation, verify the plugin is working:

### 1. Check MCP Server

```bash
# The MCP server should be listed in Claude Code's MCP servers
# Check the .mcp.json file is detected
```

### 2. Test Slash Commands

In Claude Code, try these commands:

```
/writer.start          # Should show project creation wizard
/writer.projects       # Should list projects (empty initially)
/writer.character.add  # Should show character wizard
/writer.world.rule     # Should show world rule wizard
```

### 3. Check Binary

```bash
# Verify the binary exists and runs
./bin/story-server.exe --version  # Windows
./bin/story-server --version      # macOS/Linux
```

---

## Troubleshooting

### "MCP server not found"

**Solution**: Ensure `.mcp.json` is in the project root:
```json
{
  "mcpServers": {
    "story-db": {
      "command": "bin/story-server",
      "args": [],
      "env": {
        "RUST_LOG": "info",
        "STORY_DATA_DIR": "data"
      }
    }
  }
}
```

### "Permission denied" (macOS/Linux)

**Solution**: Make the binary executable:
```bash
chmod +x bin/story-server
```

### "Binary not compatible with your system"

**Solution**: Build from source for your platform:
```bash
cd rust/story-server
cargo build --release
cp target/release/story-server ../../bin/
```

### "Rust not installed"

**Solution**: Install Rust from https://rustup.rs/
```bash
# Windows: Download and run rustup-init.exe
# macOS/Linux:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Directory Structure After Installation

```
your-projects/
└── story-writing-engine/         # Plugin folder
    ├── .claude/
    │   └── commands/writer/      # Slash commands
    │       ├── start.md
    │       ├── projects.md
    │       ├── character/
    │       └── world/
    ├── bin/
    │   └── story-server[.exe]    # MCP server binary
    ├── data/                     # SQLite databases (created automatically)
    ├── rust/story-server/        # Source code (optional)
    ├── .mcp.json                 # MCP configuration
    ├── hooks/hooks.json          # Lifecycle hooks
    └── README.md
```

---

## Updating the Plugin

### Update from Git

```bash
cd story-writing-engine
git pull origin main

# If source code changed, rebuild:
cd rust/story-server
cargo build --release
cp target/release/story-server* ../../bin/
```

### Manual Update

1. Download the latest release
2. Replace the `bin/story-server` binary
3. Replace the `.claude/commands/` directory
4. Restart Claude Code

---

## Uninstallation

To remove the plugin:

1. Delete the `story-writing-engine` folder
2. Your story databases in `data/` will be removed
3. **Backup first** if you have stories you want to keep!

### Backup Your Stories

```bash
# Before uninstalling, backup your databases:
cp -r data/ ~/Documents/story-backups/
```

---

## Platform-Specific Notes

### Windows
- Binary name: `story-server.exe`
- Path separator: `\`
- Pre-built binary included for x86_64

### macOS
- Binary name: `story-server`
- May need to allow unsigned binary in Security Settings
- Build for your architecture (Intel or Apple Silicon)

### Linux
- Binary name: `story-server`
- Requires glibc 2.31+ (Ubuntu 20.04+, Debian 11+)
- Build from source for other distributions

---

## Next Steps

After installation:

1. **Read the README** - Learn about available features
2. **Try the Quick Start** - Create your first project
3. **Explore Commands** - Type `/writer.` and press TAB to see all commands
4. **Join the Community** - Report issues on GitHub

---

## Support

- **Issues**: https://github.com/YOUR_USERNAME/story-writing-engine/issues
- **Discussions**: https://github.com/YOUR_USERNAME/story-writing-engine/discussions
- **Documentation**: See README.md for full feature list

---

**Enjoy writing your stories!** ✨
