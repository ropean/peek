# C# Hash256 Bootstrapper
# Checks for .NET 8 runtime and downloads it if needed

param(
    [switch]$NoInstall
)

Write-Host "Checking for .NET 8 runtime..." -ForegroundColor Yellow

# Check if dotnet command exists
try {
    $dotnetVersion = & dotnet --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        # Parse version number
        $versionParts = $dotnetVersion.Split('.')
        $majorVersion = [int]$versionParts[0]

        if ($majorVersion -ge 8) {
            Write-Host ".NET 8 or higher found. Starting application..." -ForegroundColor Green
        } else {
            Write-Host ".NET version $dotnetVersion found, but .NET 8+ required." -ForegroundColor Yellow
            if (-not $NoInstall) {
                Install-DotNet
            }
        }
    } else {
        Write-Host ".NET runtime not found." -ForegroundColor Yellow
        if (-not $NoInstall) {
            Install-DotNet
        }
    }
} catch {
    Write-Host ".NET runtime not found." -ForegroundColor Yellow
    if (-not $NoInstall) {
        Install-DotNet
    }
}

function Install-DotNet {
    Write-Host "Installing .NET 8 Desktop Runtime..." -ForegroundColor Yellow

    try {
        # Download .NET 8 installer
        $installerUrl = "https://download.visualstudio.microsoft.com/download/pr/1430e1e7-4d0e-4e7e-8b0a-4b7e6b8e7b0a/dotnet-runtime-8.0.0-win-x64.exe"
        $installerPath = "$PSScriptRoot\dotnet-installer.exe"

        Write-Host "Downloading .NET 8 installer..."
        Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath -UseBasicParsing

        Write-Host "Running .NET installer..."
        Start-Process -FilePath $installerPath -ArgumentList "/quiet /norestart" -Wait

        # Clean up installer
        Remove-Item $installerPath -Force

        Write-Host ".NET 8 installed successfully!" -ForegroundColor Green

    } catch {
        Write-Host "Failed to install .NET automatically." -ForegroundColor Red
        Write-Host "Please install .NET 8 manually from: https://dotnet.microsoft.com/download" -ForegroundColor Yellow
        Read-Host "Press Enter to exit"
        exit 1
    }
}

# Start the application
Write-Host "Starting C# Hash256..." -ForegroundColor Green
$exePath = Join-Path $PSScriptRoot "publish\CSharpHash.exe"
Start-Process -FilePath $exePath
