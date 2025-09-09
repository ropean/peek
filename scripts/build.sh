#!/bin/bash
echo "Building peek ..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Executable available at: target/release/peek"
else
    echo "Build failed!"
    exit 1
fi
