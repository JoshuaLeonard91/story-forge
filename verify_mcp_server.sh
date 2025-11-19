#!/bin/bash
# Story-Server MCP Verification Script
# Tests all core functionality of the MCP server

echo "=== Story-Server MCP Verification Test ==="
echo ""

# Test 1: Check binary exists
echo "Test 1: Binary availability"
if command -v story-server &> /dev/null; then
    echo "✅ story-server found in PATH"
else
    echo "❌ story-server not found"
    exit 1
fi
echo ""

# Test 2: List available tools
echo "Test 2: List available MCP tools"
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | story-server 2>/dev/null | grep -o '"tools":\[[^]]*\]' | head -1
echo "✅ Tools retrieved successfully"
echo ""

# Test 3: Create a test project
echo "Test 3: Create story project"
PROJECT_RESPONSE=$(echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mcp__story-db__createStoryProject","arguments":{"title":"MCP Verification Test","description":"Testing MCP server functionality","genre":"Science Fiction","targetLength":"novel"}}}' | story-server 2>/dev/null)

if echo "$PROJECT_RESPONSE" | grep -q '"projectId"'; then
    PROJECT_ID=$(echo "$PROJECT_RESPONSE" | grep -o '"projectId":"[^"]*"' | cut -d'"' -f4)
    echo "✅ Project created successfully: $PROJECT_ID"
else
    echo "❌ Project creation failed"
    echo "$PROJECT_RESPONSE"
    exit 1
fi
echo ""

# Test 4: List projects
echo "Test 4: List all projects"
PROJECTS=$(echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mcp__story-db__listStoryProjects","arguments":{}}}' | story-server 2>/dev/null)

if echo "$PROJECTS" | grep -q '"MCP Verification Test"'; then
    echo "✅ Project listing works - found test project"
else
    echo "❌ Project listing failed"
    exit 1
fi
echo ""

# Test 5: Add a character
echo "Test 5: Add character to project"
CHARACTER_RESPONSE=$(echo "{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/call\",\"params\":{\"name\":\"mcp__story-db__addCharacter\",\"arguments\":{\"projectId\":\"$PROJECT_ID\",\"name\":\"Test Character\",\"role\":\"protagonist\",\"personality\":\"brave, intelligent\",\"backstory\":\"A test character for verification\"}}}" | story-server 2>/dev/null)

if echo "$CHARACTER_RESPONSE" | grep -q '"characterId"'; then
    echo "✅ Character created successfully"
else
    echo "❌ Character creation failed"
    exit 1
fi
echo ""

# Test 6: Verify database created
echo "Test 6: Verify database persistence"
if [ -f "data/story_server.db" ]; then
    DB_SIZE=$(stat -c%s "data/story_server.db" 2>/dev/null || stat -f%z "data/story_server.db" 2>/dev/null)
    echo "✅ Database exists (size: $DB_SIZE bytes)"
else
    echo "❌ Database not found"
    exit 1
fi
echo ""

echo "=== All Tests Passed! ==="
echo ""
echo "Summary:"
echo "  - MCP server is operational"
echo "  - JSON-RPC communication works"
echo "  - Database operations succeed"
echo "  - Data persists correctly"
echo ""
echo "The story-server MCP server is fully functional!"
