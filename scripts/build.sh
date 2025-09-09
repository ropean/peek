#!/bin/bash
echo "Building Peek HTTP Status Checker..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Executable available at: target/release/peek"
else
    echo "Build failed!"
    exit 1
fi
