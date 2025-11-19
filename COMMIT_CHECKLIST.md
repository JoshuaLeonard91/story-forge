# Commit Safety Checklist

Before pushing to GitHub, verify these files are **NOT** being committed:

## ✗ NEVER COMMIT (User Data & Generated Files)

### User's Creative Work
- [ ] `stories/` - User's story content (novels, scenes)
- [ ] `data/*.db` - User's database files
- [ ] `test_data/` - Test data files

### Speckit & Claude Generated Files
- [ ] `specs/` - Specification documents
- [ ] `.specify/` - Speckit working directory
- [ ] `CLAUDE.md` - Auto-generated project guidelines
- [ ] `plan.md`, `tasks.md`, `spec.md` - Planning files
- [ ] `.claude/todos/` - Todo tracking files
- [ ] `.claude/settings.local.json` - User-specific settings

### Logs & Temporary Files
- [ ] `*.log` - All log files
- [ ] `story-server.log` - MCP server logs
- [ ] `*.backup`, `*.working` - Backup files
- [ ] `MCP_SERVER_TEST_RESULTS.md` - Test results

### Build Artifacts
- [ ] `rust/story-server/target/` - Rust build output
- [ ] `node_modules/` - NPM dependencies

## ✓ SAFE TO COMMIT (Source Code & Templates)

### Core Application Files
- [x] `bin/story-server.exe` - Pre-built binary for users
- [x] `rust/story-server/src/` - Rust source code
- [x] `rust/story-server/Cargo.toml` - Dependencies
- [x] `rust/story-server/Cargo.lock` - Locked dependencies
- [x] `package.json` - NPM package metadata

### Templates & Documentation
- [x] `templates/` - Command scaffolding templates
- [x] `.claude/commands/writer/*.md` - Writer commands (NOT .backup files)
- [x] `.claude/settings.json` - Default MCP settings template
- [x] `README.md` - Project documentation
- [x] `.gitignore` - Git ignore rules

## Verification Commands

Run these before committing:

```bash
# Check what would be committed
git status

# Verify sensitive files are ignored
git status --ignored

# Check specific patterns
git check-ignore stories/ data/*.db *.log test_data/
```

## Quick Safety Check

```bash
# This should show ONLY source code and templates, NO user data
git ls-files | grep -E "(stories|data|\.db|\.log|test_data|\.backup|MCP_SERVER_TEST)"
```

If the above command returns ANY results, DO NOT COMMIT! Those files should be ignored.
