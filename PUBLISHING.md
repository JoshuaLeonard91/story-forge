# Publishing Guide - Story Writing Engine

How to make this plugin available for others to download and install.

## Distribution Options

### 1. GitHub Repository (Recommended) ⭐

**Pros:**
- Version control
- Issue tracking
- Community engagement
- Automatic updates via `git pull`
- Free hosting

**Steps:**

1. **Create GitHub Repository**
   ```bash
   # Initialize git (if not already done)
   cd E:\BookPlugin
   git init
   git add .
   git commit -m "Initial commit: Story Writing Engine MVP"

   # Create repo on GitHub, then:
   git remote add origin https://github.com/YOUR_USERNAME/story-writing-engine.git
   git branch -M main
   git push -u origin main
   ```

2. **Tag a Release**
   ```bash
   git tag -a v0.1.0 -m "MVP Release - User Story 1 Complete"
   git push origin v0.1.0
   ```

3. **Create GitHub Release**
   - Go to GitHub → Releases → Create new release
   - Choose tag `v0.1.0`
   - Title: "Story Writing Engine v0.1.0 - MVP"
   - Description: Copy from README.md summary
   - Upload pre-built binaries:
     - `story-writing-engine-v0.1.0-Windows-x86_64.zip`
     - `story-writing-engine-v0.1.0-macOS-arm64.tar.gz`
     - `story-writing-engine-v0.1.0-Linux-x86_64.tar.gz`

4. **Users Install Via:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/story-writing-engine.git
   # Binary already included, ready to use!
   ```

---

### 2. Direct Download (ZIP File)

**Pros:**
- No git required
- Simple for non-technical users

**Steps:**

1. **Build Release Packages**
   ```bash
   # Windows
   .\scripts\build.ps1

   # macOS/Linux
   ./scripts/build.sh
   ```

2. **Upload to File Hosting**
   - GitHub Releases (recommended)
   - Google Drive / Dropbox (with public link)
   - Your own website

3. **Users Install Via:**
   - Download ZIP
   - Extract to Claude Code projects folder
   - Restart Claude Code

---

### 3. NPM Package (Future Option)

For advanced distribution, you could package as an npm module:

```json
{
  "name": "@your-username/story-writing-engine",
  "version": "0.1.0",
  "description": "AI-assisted novel writing plugin for Claude Code",
  "bin": {
    "story-server": "./bin/story-server"
  },
  "files": [
    ".claude/",
    "bin/",
    "data/",
    "hooks/",
    ".mcp.json"
  ]
}
```

Users would install via:
```bash
npm install -g @your-username/story-writing-engine
```

---

## Pre-Release Checklist

Before publishing your first release:

### Code Quality

- [ ] All tests passing (`cargo test`)
- [ ] No compiler warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] README.md complete
- [ ] INSTALLATION.md clear and tested
- [ ] QUICKSTART.md with working examples

### Documentation

- [ ] API documentation generated (`cargo doc`)
- [ ] MCP tools documented in contracts/
- [ ] Slash commands have descriptions
- [ ] Error messages are helpful

### Cross-Platform Testing

- [ ] Test on Windows
- [ ] Test on macOS (Intel and Apple Silicon if possible)
- [ ] Test on Linux (Ubuntu/Debian)

### Security

- [ ] No hardcoded credentials
- [ ] Input validation on all MCP tools
- [ ] SQL injection protection (using parameterized queries)
- [ ] No sensitive data in logs

### Legal

- [ ] LICENSE file (MIT recommended)
- [ ] Copyright notices in source files
- [ ] Third-party licenses acknowledged

---

## Creating Pre-built Binaries

### For Windows (x86_64)

```bash
# On Windows machine
cd rust/story-server
cargo build --release --target x86_64-pc-windows-msvc
```

### For macOS (Apple Silicon)

```bash
# On macOS machine (M1/M2/M3)
cd rust/story-server
cargo build --release --target aarch64-apple-darwin
```

### For macOS (Intel)

```bash
# On macOS machine (Intel)
cd rust/story-server
cargo build --release --target x86_64-apple-darwin
```

### For Linux (x86_64)

```bash
# On Linux machine or WSL
cd rust/story-server
cargo build --release --target x86_64-unknown-linux-gnu
```

### Cross-Compilation (Advanced)

Install cross-compilation tools:
```bash
cargo install cross

# Build for all platforms
cross build --release --target x86_64-pc-windows-msvc
cross build --release --target aarch64-apple-darwin
cross build --release --target x86_64-unknown-linux-gnu
```

---

## Release Process

### Version Numbering (Semantic Versioning)

- **v0.1.0** - MVP (User Story 1)
- **v0.2.0** - User Story 2 (Context-aware writing)
- **v0.3.0** - User Story 3 (Character arcs)
- **v1.0.0** - All user stories complete, production-ready

### Creating a Release

1. **Update Version**
   ```toml
   # rust/story-server/Cargo.toml
   [package]
   version = "0.1.0"  # Increment this
   ```

2. **Update Changelog**
   ```markdown
   # CHANGELOG.md

   ## [0.1.0] - 2025-11-18

   ### Added
   - Story project management (create, load, list)
   - Character management with relationships
   - World rule definition
   - Plot structure (Three-Act, Five-Act, Hero's Journey)
   - 15 MCP tools implemented
   - 4 slash commands

   ### Testing
   - 33 tests passing (100% success rate)
   ```

3. **Build Release Binaries**
   ```bash
   # Run build scripts for each platform
   ./scripts/build.sh      # macOS/Linux
   .\scripts\build.ps1     # Windows
   ```

4. **Create Git Tag**
   ```bash
   git tag -a v0.1.0 -m "MVP Release"
   git push origin v0.1.0
   ```

5. **Publish on GitHub**
   - Create release from tag
   - Upload binary archives
   - Copy release notes from CHANGELOG.md

---

## Promotion & Marketing

### Where to Share

1. **GitHub**
   - Add topics: `claude-code`, `mcp-server`, `creative-writing`, `rust`, `ai-tools`
   - Write engaging README with screenshots
   - Create issues template for bug reports
   - Enable discussions for community

2. **Reddit**
   - r/writing
   - r/nanowrimo
   - r/selfpublish
   - r/rust (technical audience)

3. **Twitter/X**
   - Tweet with demo video
   - Use hashtags: #ClaudeCode #CreativeWriting #AI #Rust

4. **Writing Communities**
   - NaNoWriMo forums
   - Writing.com
   - Absolute Write Water Cooler

5. **Product Hunt**
   - Launch when v1.0 is ready
   - Prepare demo video

### Demo Materials

Create these to showcase your plugin:

1. **Screenshot** - `/writer.start` wizard
2. **GIF** - Creating a project end-to-end
3. **Video** (2-3 minutes) - Full walkthrough
4. **Blog Post** - "How I built an AI writing assistant with Rust"

---

## Maintenance & Updates

### Responding to Issues

- Triage issues weekly
- Label: `bug`, `enhancement`, `question`, `help-wanted`
- Use issue templates

### Releasing Updates

```bash
# 1. Fix bugs or add features
# 2. Update version
# 3. Update CHANGELOG
# 4. Run tests
cargo test

# 5. Build new release
./scripts/build.sh

# 6. Tag and push
git tag v0.1.1
git push origin v0.1.1

# 7. Create GitHub release
```

### Backwards Compatibility

- Don't break existing MCP tool contracts
- Add migration scripts for database changes
- Deprecate features gradually (3 versions)

---

## Example GitHub Release Notes

```markdown
# Story Writing Engine v0.1.0 - MVP Release

The first public release of the Story Writing Engine! 🎉

## What's Included

✅ **Story Project Management**
- Create projects with customizable plot structures
- Track word counts and status
- Support for short stories to full series

✅ **Character Management**
- Detailed character profiles
- Character relationships
- State tracking

✅ **World Building**
- Define consistent world rules
- Keyword tagging for context retrieval
- Scope management (universal/regional/situational)

✅ **Plot Organization**
- Three-Act, Five-Act, Hero's Journey structures
- Hierarchical acts → chapters → scenes

## Installation

Download the appropriate binary for your platform below:

- Windows: `story-writing-engine-v0.1.0-Windows-x86_64.zip`
- macOS (Intel): `story-writing-engine-v0.1.0-macOS-x86_64.tar.gz`
- macOS (Apple Silicon): `story-writing-engine-v0.1.0-macOS-arm64.tar.gz`
- Linux: `story-writing-engine-v0.1.0-Linux-x86_64.tar.gz`

See [INSTALLATION.md](INSTALLATION.md) for detailed setup instructions.

## Quick Start

1. Extract the archive to your Claude Code projects folder
2. Restart Claude Code
3. Try `/writer.start` to create your first project!

See [QUICKSTART.md](QUICKSTART.md) for a guided tutorial.

## What's Next

- **v0.2.0**: Context-aware scene writing with selective memory
- **v0.3.0**: Character arc tracking
- **v0.4.0**: Auto-generated summaries
- **v0.5.0**: Continuity validation

## Report Issues

Found a bug? Have a feature request?
Open an issue: https://github.com/YOUR_USERNAME/story-writing-engine/issues

## Feedback Welcome!

This is the first public release. Your feedback helps improve the plugin for everyone!

---

**Built with**: Rust 🦀 | SQLite | Model Context Protocol
**License**: MIT
```

---

## Legal Considerations

### License (MIT Recommended)

Create `LICENSE` file:

```
MIT License

Copyright (c) 2025 [Your Name]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### Attribution

If using third-party code, include their licenses in `THIRD_PARTY_LICENSES.md`.

---

## Support Channels

Set up these for users:

1. **GitHub Issues** - Bug reports
2. **GitHub Discussions** - Feature requests, Q&A
3. **Discord Server** (optional) - Community chat
4. **Email** - Direct support for critical issues

---

## Success Metrics

Track these to measure adoption:

- GitHub stars ⭐
- Download counts
- Issue resolution time
- Active users (if analytics added)
- Community contributions (PRs)

---

**You're ready to publish!** 🚀

Start with a GitHub repository, create your first release, and share it with the writing community!
