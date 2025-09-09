@echo off
REM C# Hash256 Bootstrapper
REM Checks for .NET 8 runtime and downloads it if needed

echo Checking for .NET 8 runtime...

REM Check if dotnet command exists and if it's version 8 or higher
dotnet --version >nul 2>&1
if %errorlevel% neq 0 (
    echo .NET runtime not found. Installing .NET 8...
    goto :install_dotnet
)

REM Check if it's .NET 8 or higher
for /f "tokens=1,2 delims=." %%a in ('dotnet --version') do (
    if %%a geq 8 (
        echo .NET 8 or higher found. Starting application...
        goto :start_app
    )
)

:install_dotnet
echo Installing .NET 8 Desktop Runtime...
powershell -Command "& {Invoke-WebRequest -Uri 'https://download.visualstudio.microsoft.com/download/pr/1430e1e7-4d0e-4e7e-8b0a-4b7e6b8e7b0a/dotnet-runtime-8.0.0-win-x64.exe' -OutFile 'dotnet-installer.exe'}"
if %errorlevel% neq 0 (
    echo Failed to download .NET installer. Please install .NET 8 manually from https://dotnet.microsoft.com/download
    pause
    exit /b 1
)

echo Running .NET installer...
dotnet-installer.exe /quiet /norestart
if %errorlevel% neq 0 (
    echo Failed to install .NET. Please install .NET 8 manually from https://dotnet.microsoft.com/download
    pause
    exit /b 1
)

REM Clean up installer
del dotnet-installer.exe

echo .NET 8 installed successfully!

:start_app
echo Starting C# Hash256...
start "" "publish\CSharpHash.exe"
