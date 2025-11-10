#!/bin/bash
# Build script for macOS .app bundle

set -e

echo "Building Peek.app for macOS..."
echo "================================"

# Build the release binary
echo "Step 1: Building release binary..."
cargo build --release

if [ ! -f "target/release/peek" ]; then
    echo "Error: Binary not found at target/release/peek"
    exit 1
fi

echo "Step 2: Creating .app bundle structure..."

# Define the app bundle structure
APP_NAME="Peek.app"
APP_DIR="target/release/${APP_NAME}"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

# Remove old app bundle if exists
if [ -d "${APP_DIR}" ]; then
    echo "Removing old ${APP_NAME}..."
    rm -rf "${APP_DIR}"
fi

# Create directory structure
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

echo "Step 3: Copying binary..."
cp "target/release/peek" "${MACOS_DIR}/peek"
chmod +x "${MACOS_DIR}/peek"

echo "Step 4: Copying Info.plist..."
cp "assets/Info.plist" "${CONTENTS_DIR}/Info.plist"

echo "Step 5: Handling icon..."
# Check if .icns file exists, if not try to convert from .ico or .png
if [ -f "assets/peek.icns" ]; then
    echo "Using existing peek.icns"
    cp "assets/peek.icns" "${RESOURCES_DIR}/peek.icns"
elif [ -f "assets/pk.ico" ]; then
    echo "Converting pk.ico to peek.icns..."
    # Try using sips (macOS built-in tool) or iconutil
    if command -v sips &> /dev/null; then
        # Create iconset directory
        ICONSET_DIR="${RESOURCES_DIR}/peek.iconset"
        mkdir -p "${ICONSET_DIR}"

        # Convert ico to png and create multiple sizes for iconset
        # Note: This is a simplified approach. For production, use proper iconutil workflow
        sips -s format png "assets/pk.ico" --out "${ICONSET_DIR}/icon_512x512.png" -z 512 512 2>/dev/null || true
        sips -s format png "assets/pk.ico" --out "${ICONSET_DIR}/icon_256x256.png" -z 256 256 2>/dev/null || true
        sips -s format png "assets/pk.ico" --out "${ICONSET_DIR}/icon_128x128.png" -z 128 128 2>/dev/null || true
        sips -s format png "assets/pk.ico" --out "${ICONSET_DIR}/icon_32x32.png" -z 32 32 2>/dev/null || true
        sips -s format png "assets/pk.ico" --out "${ICONSET_DIR}/icon_16x16.png" -z 16 16 2>/dev/null || true

        # Try to create .icns from iconset
        if command -v iconutil &> /dev/null; then
            iconutil -c icns "${ICONSET_DIR}" -o "${RESOURCES_DIR}/peek.icns"
            rm -rf "${ICONSET_DIR}"
        else
            echo "Warning: iconutil not found. Icon conversion incomplete."
            rm -rf "${ICONSET_DIR}"
        fi
    else
        echo "Warning: sips not found. Cannot convert icon. Please provide peek.icns manually."
    fi
else
    echo "Warning: No icon file found. App will use default icon."
fi

echo "Step 6: Compressing binary with UPX (if available)..."
if command -v upx &> /dev/null; then
    echo "Compressing with UPX..."
    upx --best --lzma "${MACOS_DIR}/peek" 2>/dev/null || echo "UPX compression skipped (already compressed or failed)"
else
    echo "UPX not found. Skipping compression."
    echo "Install with: brew install upx"
fi

# Calculate file sizes
BINARY_SIZE=$(stat -f%z "${MACOS_DIR}/peek" 2>/dev/null || stat -c%s "${MACOS_DIR}/peek" 2>/dev/null)
BINARY_SIZE_MB=$(awk "BEGIN {printf \"%.2f\", $BINARY_SIZE / 1048576}")

echo ""
echo "================================"
echo "✅ Build successful!"
echo "================================"
echo "App bundle: ${APP_DIR}"
echo "Binary size: ${BINARY_SIZE_MB} MB"
echo ""
echo "To run the app:"
echo "  open ${APP_DIR}"
echo ""
echo "To install to Applications:"
echo "  cp -r ${APP_DIR} /Applications/"
echo ""
echo "Note: The CLI functionality is still available by running:"
echo "  ${MACOS_DIR}/peek cli <url>"
echo "================================"
