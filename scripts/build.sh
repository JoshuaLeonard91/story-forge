#!/bin/bash
# Build script for Story Writing Engine
# Creates release binaries for multiple platforms

set -e

echo "📦 Story Writing Engine - Build Script"
echo "======================================"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get version from Cargo.toml
VERSION=$(grep "^version" rust/story-server/Cargo.toml | head -1 | cut -d '"' -f 2)
echo -e "${BLUE}Version: ${VERSION}${NC}"

# Determine target platform
PLATFORM=$(uname -s)
ARCH=$(uname -m)

echo ""
echo "Platform: $PLATFORM $ARCH"
echo ""

# Build for current platform
echo -e "${BLUE}Building for current platform...${NC}"
cd rust/story-server

cargo build --release

# Copy binary to bin/
if [ "$PLATFORM" == "Darwin" ] || [ "$PLATFORM" == "Linux" ]; then
    cp target/release/story-server ../../bin/story-server
    chmod +x ../../bin/story-server
    echo -e "${GREEN}✓ Binary copied to bin/story-server${NC}"
elif [[ "$PLATFORM" == MINGW* ]] || [[ "$PLATFORM" == MSYS* ]]; then
    cp target/release/story-server.exe ../../bin/story-server.exe
    echo -e "${GREEN}✓ Binary copied to bin/story-server.exe${NC}"
fi

cd ../..

# Run tests
echo ""
echo -e "${BLUE}Running tests...${NC}"
cd rust/story-server
cargo test --release
cd ../..

# Create release package
echo ""
echo -e "${BLUE}Creating release package...${NC}"

RELEASE_DIR="releases/story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}"
mkdir -p "$RELEASE_DIR"

# Copy essential files
cp -r .claude "$RELEASE_DIR/"
cp -r bin "$RELEASE_DIR/"
mkdir -p "$RELEASE_DIR/data"
cp -r hooks "$RELEASE_DIR/"
cp .mcp.json "$RELEASE_DIR/"
cp README.md "$RELEASE_DIR/"
cp INSTALLATION.md "$RELEASE_DIR/"
cp QUICKSTART.md "$RELEASE_DIR/"
cp LICENSE "$RELEASE_DIR/" 2>/dev/null || echo "MIT" > "$RELEASE_DIR/LICENSE"

# Create archive
cd releases
if command -v zip &> /dev/null; then
    zip -r "story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}.zip" "story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}"
    echo -e "${GREEN}✓ Created ZIP archive${NC}"
elif command -v tar &> /dev/null; then
    tar -czf "story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}.tar.gz" "story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}"
    echo -e "${GREEN}✓ Created TAR.GZ archive${NC}"
fi
cd ..

echo ""
echo -e "${GREEN}✅ Build complete!${NC}"
echo ""
echo "Binary: bin/story-server"
echo "Release: releases/story-writing-engine-v${VERSION}-${PLATFORM}-${ARCH}"
echo ""
echo "To install:"
echo "  1. Copy the release folder to your Claude Code projects"
echo "  2. Restart Claude Code"
echo "  3. Try /writer.start"
