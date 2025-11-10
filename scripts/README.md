# Build Scripts

This directory contains build scripts for different platforms.

## Scripts Overview

### `build.sh`
Main build script for macOS and Linux.

**macOS behavior:**
- Detects macOS platform automatically
- Offers choice between:
  1. Building .app bundle (default, recommended for GUI)
  2. Building binary only (for CLI usage)
- Delegates to `build-macos-app.sh` if .app bundle is selected

**Linux behavior:**
- Builds standard executable
- Applies UPX compression if available
- Shows file size statistics

**Usage:**
```bash
./scripts/build.sh
```

### `build-macos-app.sh`
Specialized script for building macOS .app bundles.

**Features:**
- Creates proper .app bundle structure
- Copies Info.plist and icon files
- Applies UPX compression
- Attempts icon conversion if .icns not found

**Output:**
- Creates `target/release/Peek.app`
- Fully functional macOS application
- Can be installed to /Applications/

**Usage:**
```bash
./scripts/build-macos-app.sh
```

**Installation:**
```bash
cp -r target/release/Peek.app /Applications/
```

### `build.cmd`
Windows build script with UPX compression support.

**Features:**
- Builds Windows executable
- Applies UPX compression if available
- Shows file size before and after compression

**Usage:**
```cmd
scripts\build.cmd
```

### `convert-icon-to-icns.sh`
Converts icon files to macOS .icns format.

**Supported input formats:**
- .ico
- .png
- .svg

**Requirements:**
- macOS only (uses `sips` and `iconutil`)
- Generates all required icon sizes for Retina displays

**Usage:**
```bash
./scripts/convert-icon-to-icns.sh assets/pk.ico
```

**Output:**
- Creates `assets/peek.icns`
- Suitable for use in .app bundles

### `run.sh` / `run.cmd`
Quick launch scripts for development.

**Usage:**
```bash
./scripts/run.sh      # macOS/Linux
scripts\run.cmd       # Windows
```

### `deploy.sh`
Deployment helper script (if applicable).

## Platform-Specific Notes

### macOS .app Bundle Structure

The generated .app bundle follows standard macOS application structure:

```
Peek.app/
├── Contents/
│   ├── Info.plist          # Application metadata
│   ├── MacOS/
│   │   └── peek            # Executable binary
│   └── Resources/
│       └── peek.icns       # Application icon
```

### CLI Usage from .app Bundle

Even when built as a .app bundle, CLI functionality remains available:

```bash
# From anywhere after installing to /Applications/
/Applications/Peek.app/Contents/MacOS/peek cli aceapp.dev

# Or from build directory
./target/release/Peek.app/Contents/MacOS/peek cli aceapp.dev
```

### Windows GUI + CLI Hybrid

The Windows build uses a special configuration that:
- Uses Windows subsystem (no console by default)
- Dynamically attaches console when CLI args detected
- Provides seamless experience for both GUI and CLI

### Compression with UPX

All scripts support optional UPX compression:

**Install UPX:**
- macOS: `brew install upx`
- Linux: `apt install upx` or `brew install upx`
- Windows: `scoop install upx` or `choco install upx`

**Benefits:**
- Reduces executable size by 50-70%
- No runtime performance penalty
- Works on all platforms

## Development Workflow

**Quick development build:**
```bash
cargo run --release
```

**Production build for distribution:**
```bash
# macOS
./scripts/build-macos-app.sh

# Windows
scripts\build.cmd

# Linux
./scripts/build.sh
```

## Troubleshooting

### macOS: Icon not showing
1. Ensure `assets/peek.icns` exists
2. Run icon conversion: `./scripts/convert-icon-to-icns.sh assets/pk.ico`
3. Rebuild: `./scripts/build-macos-app.sh`

### Windows: Console window appears when double-clicking
- This should not happen if built correctly
- Check that `build.rs` is setting the Windows subsystem
- Verify `windows_subsystem = "windows"` in `main.rs`

### UPX compression fails
- Check if binary is already compressed: `upx -t <binary>`
- Some antivirus software may interfere
- UPX is optional; builds work fine without it

### Linux: Binary won't run
- Check permissions: `chmod +x target/release/peek`
- Verify dependencies are installed
- Check with: `ldd target/release/peek`
