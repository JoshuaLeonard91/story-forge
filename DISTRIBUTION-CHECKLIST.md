# Distribution Checklist - Make Your Plugin Downloadable

Quick checklist to publish the Story Writing Engine so others can download and install it.

## ✅ Option 1: GitHub (Fastest & Recommended)

### Step 1: Create GitHub Repository (5 minutes)

1. Go to https://github.com/new
2. Repository name: `story-writing-engine`
3. Description: "AI-assisted novel writing plugin for Claude Code with Rust MCP server"
4. Make it **Public**
5. Skip "Initialize with README" (we have one)
6. Click "Create repository"

### Step 2: Push Your Code (2 minutes)

```bash
# In E:\BookPlugin directory
git init
git add .
git commit -m "Initial commit: Story Writing Engine MVP v0.1.0"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/story-writing-engine.git
git push -u origin main
```

### Step 3: Create a Release (3 minutes)

1. On GitHub, go to your repository
2. Click "Releases" → "Create a new release"
3. Click "Choose a tag" → Type `v0.1.0` → "Create new tag"
4. Release title: `Story Writing Engine v0.1.0 - MVP`
5. Description: Copy this:

```markdown
# Story Writing Engine v0.1.0 - MVP Release

AI-assisted novel writing plugin for Claude Code.

## Features
✅ Story project management
✅ Character profiles with relationships
✅ World building with rules
✅ Plot structures (3-act, 5-act, Hero's Journey)
✅ 15 MCP tools implemented
✅ SQLite database with full-text search

## Installation

**Windows:**
1. Download `story-writing-engine-Windows.zip` below
2. Extract to your Claude Code projects folder
3. Binary is pre-built in `bin/story-server.exe`
4. Restart Claude Code
5. Try `/writer.start`

**macOS/Linux:**
See INSTALLATION.md for build instructions.

## Quick Start
See QUICKSTART.md for a guided tutorial.

Built with Rust 🦀 | MIT License
```

6. **Attach the binary**: Drag and drop `bin/story-server.exe` as an asset
7. Click "Publish release"

### Step 4: Share the Link

**Your plugin is now downloadable!** Share this URL:
```
https://github.com/YOUR_USERNAME/story-writing-engine
```

Users can install via:
```bash
git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
```

**Done! ✅**

---

## ✅ Option 2: Pre-built ZIP for Direct Download

### Step 1: Create Release Package (5 minutes)

```powershell
# Run the build script
.\scripts\build.ps1

# This creates: releases/story-writing-engine-v0.1.0-Windows-x86_64.zip
```

### Step 2: Upload to GitHub Release

1. Follow "Option 1" steps 1-3 above
2. In Step 3, attach the ZIP file from `releases/` folder
3. Users download ZIP directly

---

## 📦 What Users Get

When users download your plugin, they get:

```
story-writing-engine/
├── .claude/
│   └── commands/writer/    # Slash commands (ready to use)
├── bin/
│   └── story-server.exe    # Pre-built binary (4.0 MB)
├── data/                   # SQLite databases (auto-created)
├── hooks/hooks.json        # Lifecycle hooks
├── .mcp.json               # MCP server config
├── README.md               # Feature list
├── INSTALLATION.md         # Setup guide
└── QUICKSTART.md           # Tutorial
```

**No compilation needed!** The binary is pre-built.

---

## 🚀 Installation for Users (Their Steps)

### Method 1: Git Clone (Developers)

```bash
# 1. Clone the repository
git clone https://github.com/YOUR_USERNAME/story-writing-engine.git

# 2. Open in Claude Code
# (Claude Code automatically detects .mcp.json)

# 3. Try it!
/writer.start
```

### Method 2: ZIP Download (Everyone)

1. Go to GitHub releases page
2. Download `story-writing-engine-v0.1.0-Windows-x86_64.zip`
3. Extract to `Documents/Claude Code Projects/story-writing-engine/`
4. Restart Claude Code
5. Try `/writer.start`

**That's it!** No Rust installation, no compilation.

---

## 🔄 Updating Your Plugin Later

When you add new features:

```bash
# 1. Make changes and commit
git add .
git commit -m "Add User Story 2: Context-aware writing"

# 2. Rebuild binary
.\scripts\build.ps1

# 3. Create new release
git tag v0.2.0
git push origin v0.2.0

# 4. Create GitHub release with new binary
```

Users update via:
```bash
cd story-writing-engine
git pull
```

---

## 📋 Pre-Flight Checklist

Before making it public, verify:

- [ ] Binary works: `./bin/story-server.exe` runs without errors
- [ ] Tests pass: `cd rust/story-server && cargo test`
- [ ] README.md has clear feature list
- [ ] INSTALLATION.md has step-by-step instructions
- [ ] QUICKSTART.md has working example
- [ ] LICENSE file exists (MIT)
- [ ] .gitignore prevents committing data/*.db files

---

## 💡 Quick Test

**Test your distribution:**

1. Create a fresh folder: `C:\Temp\test-install`
2. Clone your repo there
3. Open in Claude Code
4. Try `/writer.start`
5. If it works, you're ready to share! ✅

---

## 🎯 Where to Share

Once published:

1. **Reddit**: r/writing, r/selfpublish
2. **Twitter/X**: #ClaudeCode #CreativeWriting
3. **GitHub Topics**: Add tags: `claude-code`, `mcp-server`, `creative-writing`
4. **Writing Forums**: NaNoWriMo, Absolute Write

---

## 🆘 Common Issues

### "Users can't find the binary"

**Fix**: Ensure `bin/story-server.exe` is committed to git:
```bash
git add -f bin/story-server.exe
git commit -m "Add pre-built binary"
git push
```

### "MCP server not starting"

**Fix**: Check `.mcp.json` uses relative path:
```json
{
  "mcpServers": {
    "story-db": {
      "command": "bin/story-server",  # NOT absolute path
      ...
    }
  }
}
```

### "Binary won't run on other machines"

**Fix**: Build with `--release` flag:
```bash
cargo build --release
```

---

## ✅ Success Criteria

You'll know it's working when:

- [ ] Someone else clones your repo
- [ ] They open it in Claude Code
- [ ] They type `/writer.start`
- [ ] It works without errors! 🎉

---

**Ready to go!** Follow Option 1 (GitHub) and you'll have a downloadable plugin in 10 minutes.

Need help? See PUBLISHING.md for advanced options.
