#!/usr/bin/env node

/**
 * HTTP MCP Server for Story Forge
 *
 * This server wraps the Rust story-server binary and provides an HTTP interface
 * for Claude Code to connect to. Much more reliable than stdio transport.
 */

const express = require('express');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Configuration
const PORT = process.env.MCP_PORT || 3000;
const HOST = process.env.MCP_HOST || 'localhost';

// Find the Rust binary
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

// Spawn the Rust binary once and keep it alive
let storyServer = null;
let requestId = 0;
const pendingRequests = new Map();
let buffer = '';

function startStoryServer() {
  console.log(`Starting story-server: ${STORY_SERVER_BIN}`);

  storyServer = spawn(STORY_SERVER_BIN, [], {
    env: {
      ...process.env,
      RUST_LOG,
      STORY_DATA_DIR
    },
    stdio: ['pipe', 'pipe', 'inherit']
  });

  // Handle responses from Rust binary
  storyServer.stdout.on('data', (data) => {
    buffer += data.toString();

    // Process complete JSON messages (newline-delimited)
    const lines = buffer.split('\n');
    buffer = lines.pop() || '';

    for (const line of lines) {
      if (!line.trim()) continue;

      try {
        const response = JSON.parse(line);

        if (response.id && pendingRequests.has(response.id)) {
          const { resolve } = pendingRequests.get(response.id);
          pendingRequests.delete(response.id);
          resolve(response);
        }
      } catch (err) {
        console.error('Failed to parse response:', line, err);
      }
    }
  });

  storyServer.on('error', (err) => {
    console.error('Story server error:', err);
  });

  storyServer.on('exit', (code) => {
    console.error('Story server exited with code:', code);
    storyServer = null;

    // Reject all pending requests
    for (const [id, { reject }] of pendingRequests.entries()) {
      reject(new Error('Story server exited'));
      pendingRequests.delete(id);
    }
  });
}

// Send request to Rust binary
function sendToStoryServer(method, params) {
  return new Promise((resolve, reject) => {
    if (!storyServer) {
      startStoryServer();
    }

    const id = ++requestId;
    const request = {
      jsonrpc: '2.0',
      id,
      method,
      params: params || {}
    };

    // Set timeout for request
    const timeout = setTimeout(() => {
      if (pendingRequests.has(id)) {
        pendingRequests.delete(id);
        reject(new Error('Request timeout'));
      }
    }, 30000); // 30 second timeout

    pendingRequests.set(id, {
      resolve: (response) => {
        clearTimeout(timeout);
        resolve(response);
      },
      reject: (err) => {
        clearTimeout(timeout);
        reject(err);
      }
    });

    storyServer.stdin.write(JSON.stringify(request) + '\n');
  });
}

// Create Express app
const app = express();
app.use(express.json());

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
    status: 'ok',
    server: 'story-forge-mcp',
    version: '0.1.0',
    storyServerRunning: storyServer !== null
  });
});

// MCP endpoint - handles all JSON-RPC requests
app.post('/mcp', async (req, res) => {
  try {
    const { method, params, id } = req.body;

    if (!method) {
      return res.status(400).json({
        jsonrpc: '2.0',
        id: id || null,
        error: {
          code: -32600,
          message: 'Invalid Request: method is required'
        }
      });
    }

    console.log(`[${new Date().toISOString()}] ${method}`);

    // Forward to story server
    const response = await sendToStoryServer(method, params);

    // Return the response
    res.json(response);

  } catch (error) {
    console.error('Error handling request:', error);
    res.status(500).json({
      jsonrpc: '2.0',
      id: req.body.id || null,
      error: {
        code: -32603,
        message: error.message || 'Internal error'
      }
    });
  }
});

// CORS support
app.use((req, res, next) => {
  res.header('Access-Control-Allow-Origin', '*');
  res.header('Access-Control-Allow-Headers', 'Content-Type');
  res.header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  if (req.method === 'OPTIONS') {
    return res.sendStatus(200);
  }
  next();
});

// Start the server
function start() {
  startStoryServer();

  app.listen(PORT, HOST, () => {
    console.log(`\nStory Forge MCP HTTP Server`);
    console.log(`============================`);
    console.log(`Listening on: http://${HOST}:${PORT}`);
    console.log(`MCP endpoint: http://${HOST}:${PORT}/mcp`);
    console.log(`Health check: http://${HOST}:${PORT}/health`);
    console.log(`\nReady for connections!\n`);
  });
}

// Handle shutdown
process.on('SIGTERM', () => {
  console.log('Shutting down...');
  if (storyServer) {
    storyServer.kill();
  }
  process.exit(0);
});

process.on('SIGINT', () => {
  console.log('\nShutting down...');
  if (storyServer) {
    storyServer.kill();
  }
  process.exit(0);
});

start();
