@echo off
echo Building Peek HTTP Status Checker...
cd /d "%~dp0.."
cargo build --release
if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    echo Executable available at: target\release\peek.exe
) else (
    echo Build failed!
)
