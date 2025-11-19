#!/usr/bin/env node

/**
 * Simple stdio proxy for the Rust story-server
 *
 * This wrapper just spawns the Rust binary and pipes stdio through.
 * No MCP SDK - just pure stdio proxying.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Find the story-server binary
function findStoryServerBinary() {
  if (process.env.STORY_SERVER_BIN) {
    return process.env.STORY_SERVER_BIN;
  }

  const scriptDir = __dirname;
  const binaryName = process.platform === 'win32' ? 'story-server.exe' : 'story-server';
  const localBinary = path.join(scriptDir, binaryName);

  if (fs.existsSync(localBinary)) {
    return localBinary;
  }

  return binaryName;
}

const STORY_SERVER_BIN = findStoryServerBinary();
const RUST_LOG = process.env.RUST_LOG || 'error';
const STORY_DATA_DIR = process.env.STORY_DATA_DIR || 'data';

// Spawn the Rust binary
const storyServer = spawn(STORY_SERVER_BIN, [], {
  env: {
    ...process.env,
    RUST_LOG,
    STORY_DATA_DIR
  },
  stdio: ['pipe', 'pipe', 'inherit']
});

// Pipe stdin to Rust binary
process.stdin.pipe(storyServer.stdin);

// Pipe Rust binary output to stdout
storyServer.stdout.pipe(process.stdout);

// Handle errors
storyServer.on('error', (err) => {
  console.error('Failed to start story-server:', err.message);
  process.exit(1);
});

storyServer.on('exit', (code) => {
  process.exit(code || 0);
});

// Forward termination signals
process.on('SIGTERM', () => storyServer.kill('SIGTERM'));
process.on('SIGINT', () => storyServer.kill('SIGINT'));
