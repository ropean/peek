# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.2] - 2025-11-04

### Fixed

- **Windows GUI**: Fixed console window appearing alongside GUI in release builds
  - Added proper Windows subsystem configuration in `build.rs`
  - Set linker flags `/SUBSYSTEM:WINDOWS` and `/ENTRY:mainCRTStartup`
  - GUI now launches without any console window
  - Console is still dynamically attached when CLI arguments are present
- Added missing `wincon` feature to `winapi` dependency for console attachment support

### Changed

- Improved build system configuration for Windows GUI applications
- Enhanced cross-platform compatibility with proper conditional compilation

## [1.1.1] - 2025-11-04

### Added

- Multi-platform build support for automatic releases
  - Windows x64 and x86
  - macOS Intel (x64) and Apple Silicon (ARM64)
  - Linux x64 and x86
- Automated GitHub Actions workflow for release builds
- UPX compression for Windows and Linux binaries
- Strip optimization for macOS binaries
- Automated Scoop bucket updates after successful releases

### Changed

- Improved build scripts with file size reporting
- Enhanced cross-platform compilation support

## [1.0.0] - 2025-11-04

### Initial Features

- GitHub Actions integration
- UPX compression support for executable files
- Enhanced GUI with improved styling
- Request button with better visual design
- Application icon support (.ico for Windows GUI)
- SEO optimization with dedicated documentation
- Modern website (v2) with horizontal logo
- HTTP status code reference documentation
- Comprehensive build and deployment scripts

### Features

- HTTP request inspector with modern GUI
- Request and response visualization
- Network utilities and HTTP client functionality
- Cross-platform support (Windows, macOS, Linux)

### Documentation

- SEO image guidelines
- Shell script formatting tools
- AutoHotkey resize utility
- Deployment automation scripts

---

## Version History

- **1.1.2** - Fixed Windows console window issue in GUI mode
- **1.1.1** - Multi-platform release automation
- **1.0.0** - Initial release with core HTTP inspection features
