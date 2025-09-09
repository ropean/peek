# C# Hash256 - Manual Release Creation Script
# This script helps create releases when GitHub Actions token permissions fail

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,

    [Parameter(Mandatory=$false)]
    [string]$ReleaseNotes = "Automated release of C# Hash256 v$Version",

    [switch]$Draft,
    [switch]$PreRelease
)

Write-Host "🚀 C# Hash256 Manual Release Creator" -ForegroundColor Cyan
Write-Host "Version: $Version" -ForegroundColor Yellow
Write-Host ""

# Check if we're in the right directory
if (!(Test-Path "CSharpHash\CSharpHash.csproj")) {
    Write-Host "❌ Error: Please run this script from the repository root directory" -ForegroundColor Red
    exit 1
}

# Clean and build
Write-Host "🧹 Cleaning previous builds..." -ForegroundColor Yellow
if (Test-Path "publish") {
    Remove-Item -Recurse -Force "publish"
}

Write-Host "🔨 Building release..." -ForegroundColor Yellow
try {
    & "scripts\build.cmd" "fd" "release" "/p:Version=$Version"
    if ($LASTEXITCODE -ne 0) {
        throw "Build failed with exit code $LASTEXITCODE"
    }
} catch {
    Write-Host "❌ Build failed: $_" -ForegroundColor Red
    exit 1
}

# Check if executable was created
$exePath = "publish\CSharpHash.exe"
if (!(Test-Path $exePath)) {
    Write-Host "❌ Error: Executable not found at $exePath" -ForegroundColor Red
    exit 1
}

# Get file info
$fileInfo = Get-Item $exePath
$fileSize = [math]::Round($fileInfo.Length / 1KB, 2)
Write-Host "✅ Build successful!" -ForegroundColor Green
Write-Host "📁 Executable: $exePath" -ForegroundColor Cyan
Write-Host "📏 Size: $fileSize KB" -ForegroundColor Cyan
Write-Host ""

# Instructions for manual release creation
Write-Host "📋 Manual Release Creation Instructions:" -ForegroundColor Yellow
Write-Host ""
Write-Host "1. 🏷️  Create a new tag (if not already created):" -ForegroundColor White
Write-Host "   git tag v$Version" -ForegroundColor Gray
Write-Host "   git push origin v$Version" -ForegroundColor Gray
Write-Host ""
Write-Host "2. 🌐 Go to GitHub repository releases page:" -ForegroundColor White
Write-Host "   https://github.com/[your-username]/[your-repo]/releases" -ForegroundColor Gray
Write-Host ""
Write-Host "3. ➕ Click 'Create a new release'" -ForegroundColor White
Write-Host ""
Write-Host "4. 📝 Fill in the release details:" -ForegroundColor White
Write-Host "   - Tag: v$Version" -ForegroundColor Gray
Write-Host "   - Title: C# Hash256 v$Version" -ForegroundColor Gray
Write-Host "   - Description: $ReleaseNotes" -ForegroundColor Gray
if ($PreRelease) {
    Write-Host "   - Mark as pre-release: Yes" -ForegroundColor Gray
}
if ($Draft) {
    Write-Host "   - Save as draft: Yes" -ForegroundColor Gray
}
Write-Host ""
Write-Host "5. 📎 Upload the executable:" -ForegroundColor White
Write-Host "   - Drag and drop: $exePath" -ForegroundColor Gray
Write-Host "   - Or browse and select the file" -ForegroundColor Gray
Write-Host ""
Write-Host "6. 🚀 Click 'Publish release'" -ForegroundColor White
Write-Host ""

Write-Host "📁 Executable location: $(Resolve-Path $exePath)" -ForegroundColor Cyan
Write-Host ""
Write-Host "🎉 Ready for release!" -ForegroundColor Green
