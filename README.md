# peek

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A HTTP inspector. It's Rust port of the [HttpStatus](https://github.com/ropean/HttpStatus) application, providing a modern GUI for HTTP request testing and analysis.

## Features

- **HTTP/HTTPS Support**: Automatic protocol handling with SSL/TLS support
- **Multiple Request Methods**: Support for GET and POST requests
- **Redirect Control**: Option to follow or ignore redirects
- **Comprehensive Response Display**: Shows status codes, headers, and response content
- **Network Information**: Displays both client and server IP addresses
- **DNS Resolution**: Resolves domain names to IP addresses
- **Modern GUI**: Built with egui for a responsive, cross-platform interface

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Building from Source

1. Clone or download this project
2. Navigate to the project directory:
   ```bash
   cd peek
   ```

3. Build the project:

   **Option A: Using build scripts (recommended)**

   - **macOS** (creates .app bundle):
     ```bash
     ./scripts/build.sh
     ```
     This will prompt you to choose between:
     1. Building a .app bundle (GUI application)
     2. Building a binary only (for CLI use)

     Or build directly:
     ```bash
     ./scripts/build-macos-app.sh  # Creates Peek.app
     ```

   - **Windows**:
     ```cmd
     scripts\build.cmd
     ```

   - **Linux**:
     ```bash
     ./scripts/build.sh
     ```

   **Option B: Using cargo directly**
   ```bash
   cargo build --release
   ```

4. Run the application:
   - **macOS (.app bundle)**:
     ```bash
     open target/release/Peek.app
     ```
     Or install to Applications:
     ```bash
     cp -r target/release/Peek.app /Applications/
     ```

   - **macOS/Linux (binary)**:
     ```bash
     cargo run --release
     # or
     ./target/release/peek
     ```

   - **Windows**:
     ```cmd
     cargo run --release
     REM or
     target\release\peek.exe
     ```

## Usage

1. **Enter URL**: Type the target URL in the text field (e.g., `aceapp.dev`, `https://peek.aceapp.dev/health.json`)
2. **Configure Options**:
   - **SSL**: Check to force HTTPS protocol
   - **Post**: Check to use POST method instead of GET
   - **Redirect**: Check to allow automatic redirects
3. **Make Request**: Click the "Request" button or press Enter
4. **View Results**: The response will be displayed in the lower text area, including:

   - Requested URL
   - Client IP addresses
   - Server IP addresses
   - HTTP status code
   - Response headers
   - Response body/content

   ## Command-line interface (CLI)

   In addition to the GUI, `peek` provides a CLI for headless usage. The CLI reuses the same HTTP logic as the GUI and aims to be a drop-in tool for scripting and automation.

   Basic usage:

   Start GUI (default):

   ```fish
   peek
   ```

   Single request with defaults (SSL enabled by default):

   ```fish
   peek cli aceapp.dev
   ```

   Force HTTP (no SSL) and request JSON output:

   ```fish
   peek cli --no-ssl -f json aceapp.dev
   ```

   POST with body:

   ```fish
   peek cli -X POST -d '{"key":"value"}' aceapp.dev
   ```

   Batch mode: read URLs from a file (one per line) and run with concurrency 10:

   ```fish
   peek batch -c 10 -f json urls.txt
   ```

   Flags summary (selected):

   JSON output example (array):

   ```json
   [
     {
       "requested_url": "aceapp.dev",
       "response": {
         /* HttpResponse object */
       }
     },
     { "requested_url": "bad.example", "error": "timeout" }
   ]
   ```

   Notes about concurrency:

   If you'd like, I can add a few short end-to-end examples that run against a local test server (useful for CI).

## Platform-Specific Notes

### macOS

The build script for macOS creates a proper .app bundle that can be installed to the Applications folder. The .app bundle includes:
- The executable
- Application metadata (Info.plist)
- Application icon (.icns format)

**Converting Icons to .icns:**

If you need to create or update the macOS icon:

```bash
./scripts/convert-icon-to-icns.sh assets/pk.ico
```

This will create `assets/peek.icns` which is used by the .app bundle.

**Note:** The CLI functionality is still available from within the .app bundle:
```bash
./target/release/Peek.app/Contents/MacOS/peek cli <url>
```

### Windows

The Windows executable is built with a GUI subsystem, but automatically attaches to the console when CLI arguments are provided. This means:
- Double-clicking the .exe launches the GUI
- Running with arguments in terminal shows CLI output

### Linux

The Linux build creates a standard executable that supports both GUI and CLI modes.

## Architecture

The application is structured into several modules:

- `main.rs`: Application entry point and GUI setup
- `ui.rs`: User interface implementation using egui
- `http_client.rs`: HTTP request handling and response processing
- `network_utils.rs`: Network utilities for IP resolution and local network info
- `cli.rs`: Command-line interface implementation

## Dependencies

- **egui/eframe**: Modern immediate mode GUI framework
- **reqwest**: HTTP client with async support
- **tokio**: Async runtime
- **trust-dns-resolver**: DNS resolution
- **url**: URL parsing and manipulation
- **anyhow**: Error handling

## Differences from Original

This Rust implementation maintains the same core functionality as the original C# HttpStatus application while offering:

- Cross-platform compatibility (Windows, macOS, Linux)
- Modern async HTTP handling
- Memory safety through Rust's ownership system
- Better performance and resource usage
- Modern GUI framework

## License

This project is licensed under the MIT License — see the `LICENSE` file for details.
