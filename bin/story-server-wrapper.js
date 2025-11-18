#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine binary name based on platform
const isWindows = process.platform === 'win32';
const binaryName = isWindows ? 'story-server.exe' : 'story-server';
const binaryPath = path.join(__dirname, binaryName);

// Check if binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`Error: story-server binary not found at ${binaryPath}`);
  console.error('Please install story-forge properly: npm install -g story-forge');
  process.exit(1);
}

// Spawn the Rust binary and pass through all args
const child = spawn(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env
});

child.on('error', (err) => {
  console.error(`Failed to start story-server: ${err.message}`);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code || 0);
});
