#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const GITHUB_REPO = 'YOUR_USERNAME/story-forge'; // Update after publishing to GitHub
const VERSION = require('./package.json').version;

function detectPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  // Map Node.js platform names to our binary naming convention
  const platformMap = {
    'win32': { platform: 'Windows', ext: '.exe', arch: 'x86_64' },
    'darwin': { platform: 'macOS', ext: '', arch: arch === 'arm64' ? 'arm64' : 'x86_64' },
    'linux': { platform: 'Linux', ext: '', arch: 'x86_64' }
  };

  if (!platformMap[platform]) {
    console.error(`❌ Unsupported platform: ${platform}`);
    console.error('Story Forge supports Windows, macOS, and Linux');
    process.exit(1);
  }

  return platformMap[platform];
}

function ensureBinaryInTemplates() {
  const platformInfo = detectPlatform();
  const templateBinDir = path.join(__dirname, 'templates', 'bin');
  const binaryName = 'story-server' + platformInfo.ext;
  const targetPath = path.join(templateBinDir, binaryName);

  // Create templates/bin directory if it doesn't exist
  if (!fs.existsSync(templateBinDir)) {
    fs.mkdirSync(templateBinDir, { recursive: true });
  }

  // Check if binary already exists in templates
  if (fs.existsSync(targetPath)) {
    console.log('✅ Story server binary already present');
    return;
  }

  console.log('📦 Setting up story-server binary...');

  // Try to copy from local bin/ directory (for development/local install)
  const localBinaryPath = path.join(__dirname, 'bin', binaryName);
  if (fs.existsSync(localBinaryPath)) {
    console.log('📋 Copying local binary to templates...');
    fs.copyFileSync(localBinaryPath, targetPath);

    // Make executable on Unix systems
    if (platformInfo.ext === '') {
      try {
        fs.chmodSync(targetPath, 0o755);
      } catch (error) {
        console.warn('⚠️  Warning: Could not set executable permissions');
      }
    }

    console.log('✅ Binary installed successfully');
    return;
  }

  // If local binary not found, try downloading from GitHub releases
  console.log('📥 Downloading pre-built binary from GitHub...');

  const releaseUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/story-writing-engine-v${VERSION}-${platformInfo.platform}-${platformInfo.arch}.zip`;

  console.log(`⚠️  Note: Automatic download not yet implemented.`);
  console.log(`   Please download manually from: ${releaseUrl}`);
  console.log(`   Or use local binary by running: npm install from the project directory`);
}

function main() {
  try {
    console.log('\n📚 Story Forge - Post-install setup\n');
    ensureBinaryInTemplates();
    console.log('\n✨ Installation complete!\n');
    console.log('Run "story-forge init my-novel" to create your first project\n');
  } catch (error) {
    console.error(`\n❌ Installation failed: ${error.message}`);
    console.error('\nPlease report this issue at:');
    console.error(`https://github.com/${GITHUB_REPO}/issues\n`);
    process.exit(1);
  }
}

main();
