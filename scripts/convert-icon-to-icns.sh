#!/bin/bash
# Convert icon files to macOS .icns format
# Usage: ./convert-icon-to-icns.sh <input-file>

set -e

if [ $# -eq 0 ]; then
    echo "Usage: $0 <input-icon-file>"
    echo "Supported formats: .png, .ico, .svg"
    echo ""
    echo "Example: $0 assets/pk.ico"
    exit 1
fi

INPUT_FILE="$1"
OUTPUT_FILE="assets/peek.icns"

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' not found"
    exit 1
fi

echo "Converting icon to macOS .icns format..."
echo "Input: $INPUT_FILE"
echo "Output: $OUTPUT_FILE"
echo ""

# Create a temporary directory for the iconset
ICONSET_DIR="$(mktemp -d)/peek.iconset"
mkdir -p "$ICONSET_DIR"

# Function to convert and resize image
convert_size() {
    local SIZE=$1
    local SCALE=$2
    local OUTPUT_NAME="icon_${SIZE}x${SIZE}"

    if [ "$SCALE" = "2x" ]; then
        OUTPUT_NAME="${OUTPUT_NAME}@2x"
    fi

    echo "Creating ${OUTPUT_NAME}.png..."

    # Use sips (macOS built-in tool) to convert
    sips -s format png "$INPUT_FILE" --out "$ICONSET_DIR/${OUTPUT_NAME}.png" -z $SIZE $SIZE 2>/dev/null || {
        echo "Warning: Failed to create ${OUTPUT_NAME}.png"
        return 1
    }
}

# Check if we're on macOS
if [ "$(uname)" != "Darwin" ]; then
    echo "Error: This script requires macOS (uses sips and iconutil)"
    exit 1
fi

# Check if required tools are available
if ! command -v sips &> /dev/null; then
    echo "Error: sips command not found (required on macOS)"
    exit 1
fi

if ! command -v iconutil &> /dev/null; then
    echo "Error: iconutil command not found (required on macOS)"
    exit 1
fi

# Create all required icon sizes for macOS
# Standard sizes and their @2x retina versions
convert_size 16 1x
convert_size 32 2x  # 16@2x
convert_size 32 1x
convert_size 64 2x  # 32@2x
convert_size 128 1x
convert_size 256 2x # 128@2x
convert_size 256 1x
convert_size 512 2x # 256@2x
convert_size 512 1x
convert_size 1024 2x # 512@2x

echo ""
echo "Creating .icns file..."
iconutil -c icns "$ICONSET_DIR" -o "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Success! Icon created at: $OUTPUT_FILE"
    echo ""

    # Show file size
    SIZE_BYTES=$(stat -f%z "$OUTPUT_FILE" 2>/dev/null || stat -c%s "$OUTPUT_FILE" 2>/dev/null)
    SIZE_KB=$(awk "BEGIN {printf \"%.2f\", $SIZE_BYTES / 1024}")
    echo "Icon file size: ${SIZE_KB} KB"
else
    echo "Error: Failed to create .icns file"
    exit 1
fi

# Clean up
rm -rf "$(dirname "$ICONSET_DIR")"

echo ""
echo "You can now use this icon with the macOS app bundle."
