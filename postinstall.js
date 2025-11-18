#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

function main() {
  try {
    console.log('\nStory Forge - Post-install setup\n');

    // Check if binary exists in bin/ directory
    const isWindows = process.platform === 'win32';
    const binaryName = isWindows ? 'story-server.exe' : 'story-server';
    const binaryPath = path.join(__dirname, 'bin', binaryName);

    if (fs.existsSync(binaryPath)) {
      console.log('Story server binary found');
      console.log(`Location: ${binaryPath}`);
    } else {
      console.warn('Warning: story-server binary not found');
      console.warn('You may need to build from source:');
      console.warn('  cd rust/story-server && cargo build --release');
    }

    console.log('\nInstallation complete!\n');
    console.log('Usage:');
    console.log('  story-forge init my-novel    Create a new story project');
    console.log('  story-server                 Run MCP server (called by Claude Code)\n');
    console.log('Next steps:');
    console.log('  1. Run: story-forge init my-novel');
    console.log('  2. cd my-novel');
    console.log('  3. Open folder in Claude Code');
    console.log('  4. Run /writer.mcp.setup to configure the MCP server\n');

  } catch (error) {
    console.error(`\nInstallation failed: ${error.message}`);
    console.error('\nPlease report this issue at:');
    console.error('https://github.com/YOUR_USERNAME/story-forge/issues\n');
    process.exit(1);
  }
}

main();
