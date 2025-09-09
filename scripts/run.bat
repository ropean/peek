@echo off
echo Running Peek HTTP Status Checker...
cd /d "%~dp0.."
cargo run --release
