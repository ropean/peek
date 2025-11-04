#!/bin/bash
echo "Building peek ..."
cargo build --release
if [ $? -eq 0 ]; then
	echo "Build successful!"

	# Determine the executable path
	if [ -f "target/release/peek.exe" ]; then
		EXEC_PATH="target/release/peek.exe"
	else
		EXEC_PATH="target/release/peek"
	fi

	# Show file size before compression
	if [ -f "$EXEC_PATH" ]; then
		SIZE_BYTES=$(stat -f%z "$EXEC_PATH" 2>/dev/null || stat -c%s "$EXEC_PATH" 2>/dev/null)
		SIZE_MB=$(awk "BEGIN {printf \"%.2f\", $SIZE_BYTES / 1048576}")
		echo "File size before compression: ${SIZE_MB} MB"
	fi

	# Check if UPX is available
	if command -v upx &>/dev/null; then
		echo "Compressing with UPX..."
		echo ""
		upx --best --lzma "$EXEC_PATH"
		UPX_RESULT=$?
		if [ $UPX_RESULT -eq 0 ]; then
			echo ""
			echo "Compression successful!"
		else
			# Check if already compressed
			echo "Checking if file is already compressed..."
			upx -t "$EXEC_PATH"
			if [ $? -eq 0 ]; then
				echo "File already compressed with UPX"
			else
				echo "Compression failed, but build is OK"
			fi
		fi
	else
		echo ""
		echo "Note: UPX not found. Install it to reduce executable size:"
		echo "  - macOS/Linux: brew install upx  or  apt install upx"
		echo "  - Windows: scoop install upx  or  choco install upx"
		echo ""
	fi

	# Show final file size
	if [ -f "$EXEC_PATH" ]; then
		SIZE_BYTES=$(stat -f%z "$EXEC_PATH" 2>/dev/null || stat -c%s "$EXEC_PATH" 2>/dev/null)
		SIZE_MB=$(awk "BEGIN {printf \"%.2f\", $SIZE_BYTES / 1048576}")
		echo ""
		echo "==================================="
		echo "Final executable size: ${SIZE_MB} MB"
		echo "==================================="
		echo "Executable available at: $EXEC_PATH"
	fi
else
	echo "Build failed!"
	exit 1
fi
