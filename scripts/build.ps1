# Build script for Story Writing Engine (Windows PowerShell)
# Creates release binaries for Windows

param(
    [switch]$Release = $true,
    [switch]$SkipTests = $false
)

Write-Host "📦 Story Writing Engine - Build Script" -ForegroundColor Blue
Write-Host "======================================" -ForegroundColor Blue

# Get version from Cargo.toml
$cargoToml = Get-Content "rust\story-server\Cargo.toml"
$versionLine = $cargoToml | Select-String 'version = "(.+)"'
$version = $versionLine.Matches.Groups[1].Value

Write-Host "`nVersion: $version" -ForegroundColor Cyan

# Build
Write-Host "`nBuilding Rust MCP server..." -ForegroundColor Cyan
Push-Location "rust\story-server"

if ($Release) {
    cargo build --release
    $binaryPath = "target\release\story-server.exe"
} else {
    cargo build
    $binaryPath = "target\debug\story-server.exe"
}

# Copy binary to bin/
Pop-Location
Copy-Item "rust\story-server\$binaryPath" "bin\story-server.exe" -Force
Write-Host "✓ Binary copied to bin\story-server.exe" -ForegroundColor Green

# Run tests
if (-not $SkipTests) {
    Write-Host "`nRunning tests..." -ForegroundColor Cyan
    Push-Location "rust\story-server"
    cargo test --release
    Pop-Location
}

# Create release package
Write-Host "`nCreating release package..." -ForegroundColor Cyan

$platform = "Windows"
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
$releaseDir = "releases\story-writing-engine-v$version-$platform-$arch"

# Create release directory
New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null

# Copy essential files
Copy-Item -Recurse -Force ".claude" "$releaseDir\"
Copy-Item -Recurse -Force "bin" "$releaseDir\"
New-Item -ItemType Directory -Force -Path "$releaseDir\data" | Out-Null
Copy-Item -Recurse -Force "hooks" "$releaseDir\"
Copy-Item -Force ".mcp.json" "$releaseDir\"
Copy-Item -Force "README.md" "$releaseDir\"
Copy-Item -Force "INSTALLATION.md" "$releaseDir\"
Copy-Item -Force "QUICKSTART.md" "$releaseDir\"

# Create LICENSE if it doesn't exist
if (-not (Test-Path "LICENSE")) {
    "MIT" | Out-File -FilePath "$releaseDir\LICENSE"
}

# Create ZIP archive
$zipPath = "releases\story-writing-engine-v$version-$platform-$arch.zip"
if (Test-Path $zipPath) {
    Remove-Item $zipPath
}

Add-Type -AssemblyName System.IO.Compression.FileSystem
[System.IO.Compression.ZipFile]::CreateFromDirectory($releaseDir, $zipPath)
Write-Host "✓ Created ZIP archive" -ForegroundColor Green

Write-Host "`n✅ Build complete!" -ForegroundColor Green
Write-Host "`nBinary: bin\story-server.exe" -ForegroundColor White
Write-Host "Release: $zipPath" -ForegroundColor White
Write-Host "`nTo install:" -ForegroundColor Yellow
Write-Host "  1. Extract the ZIP to your Claude Code projects folder" -ForegroundColor White
Write-Host "  2. Restart Claude Code" -ForegroundColor White
Write-Host "  3. Try /writer.start" -ForegroundColor White
