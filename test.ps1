# ArcScript Test Script for Windows
# Quick test script for ArcScript

Write-Host "=== ArcScript Test Suite ===" -ForegroundColor Cyan
Write-Host ""

# Test 1: Check build
Write-Host "Test 1: Building ArcScript..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "Build successful!" -ForegroundColor Green
Write-Host ""

# Test 2: Run example files
Write-Host "Test 2: Running example files..." -ForegroundColor Yellow
$exampleFiles = Get-ChildItem -Path "examples\*.arc"
foreach ($file in $exampleFiles) {
    Write-Host "Running $($file.Name)..." -ForegroundColor Cyan
    cargo run --quiet $file.FullName
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  Failed!" -ForegroundColor Red
    } else {
        Write-Host "  Success!" -ForegroundColor Green
    }
    Write-Host ""
}

# Test 3: Unit tests
Write-Host "Test 3: Running unit tests..." -ForegroundColor Yellow
cargo test
if ($LASTEXITCODE -ne 0) {
    Write-Host "Tests failed!" -ForegroundColor Red
    exit 1
}
Write-Host "All tests passed!" -ForegroundColor Green
Write-Host ""

Write-Host "=== All tests complete ===" -ForegroundColor Cyan
