#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const TEMPLATES_DIR = path.join(__dirname, '..', 'templates');
const VERSION = require('../package.json').version;

function showHelp() {
  console.log(`
╔═══════════════════════════════════════════════╗
║           Story Forge v${VERSION}              ║
║   AI-Assisted Novel Writing for Claude Code   ║
╚═══════════════════════════════════════════════╝

Usage:
  story-forge init <project-name>    Create a new story project
  story-forge --version              Show version
  story-forge --help                 Show this help

Examples:
  story-forge init my-novel
  story-forge init "The Last Kingdom"

After initialization:
  1. Open the project folder in Claude Code
  2. Try /writer.start to create your first story!
`);
}

function showVersion() {
  console.log(`story-forge v${VERSION}`);
}

function initProject(projectName) {
  if (!projectName) {
    console.error('❌ Error: Project name is required');
    console.log('\nUsage: story-forge init <project-name>');
    process.exit(1);
  }

  const targetDir = path.resolve(process.cwd(), projectName);

  // Check if directory already exists
  if (fs.existsSync(targetDir)) {
    console.error(`❌ Error: Directory "${projectName}" already exists`);
    process.exit(1);
  }

  console.log(`\n📚 Creating story project: ${projectName}`);
  console.log(`📁 Location: ${targetDir}\n`);

  try {
    // Create project directory
    fs.mkdirSync(targetDir, { recursive: true });

    // Copy template files
    console.log('📋 Copying template files...');
    copyRecursive(TEMPLATES_DIR, targetDir);

    // Create data directory
    const dataDir = path.join(targetDir, 'data');
    if (!fs.existsSync(dataDir)) {
      fs.mkdirSync(dataDir, { recursive: true });
    }

    // Update .mcp.json with project-specific settings
    const mcpConfigPath = path.join(targetDir, '.mcp.json');
    if (fs.existsSync(mcpConfigPath)) {
      const mcpConfig = JSON.parse(fs.readFileSync(mcpConfigPath, 'utf8'));
      // Ensure data directory is set correctly
      if (mcpConfig.mcpServers && mcpConfig.mcpServers['story-db']) {
        mcpConfig.mcpServers['story-db'].env.STORY_DATA_DIR = 'data';
      }
      fs.writeFileSync(mcpConfigPath, JSON.stringify(mcpConfig, null, 2));
    }

    console.log('\n✅ Project created successfully!\n');
    console.log('Next steps:');
    console.log(`  1. cd ${projectName}`);
    console.log('  2. Open folder in Claude Code');
    console.log('  3. Try /writer.start to create your first story!\n');
    console.log('📖 See QUICKSTART.md for a guided tutorial');

  } catch (error) {
    console.error(`\n❌ Error creating project: ${error.message}`);
    process.exit(1);
  }
}

function copyRecursive(src, dest) {
  if (!fs.existsSync(src)) {
    console.warn(`⚠️  Warning: Template directory not found: ${src}`);
    return;
  }

  const stats = fs.statSync(src);

  if (stats.isDirectory()) {
    // Create directory
    if (!fs.existsSync(dest)) {
      fs.mkdirSync(dest, { recursive: true });
    }

    // Copy contents
    const files = fs.readdirSync(src);
    for (const file of files) {
      copyRecursive(path.join(src, file), path.join(dest, file));
    }
  } else {
    // Copy file
    fs.copyFileSync(src, dest);
  }
}

// Parse command line arguments
function main() {
  const args = process.argv.slice(2);

  if (args.length === 0 || args[0] === '--help' || args[0] === '-h') {
    showHelp();
    return;
  }

  if (args[0] === '--version' || args[0] === '-v') {
    showVersion();
    return;
  }

  if (args[0] === 'init') {
    initProject(args[1]);
    return;
  }

  console.error(`❌ Unknown command: ${args[0]}`);
  console.log('\nRun "story-forge --help" for usage information');
  process.exit(1);
}

main();
