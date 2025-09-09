@echo off
echo Running peek ...
cd /d "%~dp0.."
cargo run --release
