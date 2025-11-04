# peek

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A HTTP inspector. It's Rust port of the HttpStatus application, providing a modern GUI for HTTP request testing and analysis.

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
   ```bash
   cargo build --release
   ```
4. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

1. **Enter URL**: Type the target URL in the text field (e.g., `example.com`, `https://api.github.com`)
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
   peek cli example.com
   ```

   Force HTTP (no SSL) and request JSON output:

   ```fish
   peek cli --no-ssl -f json example.com
   ```

   POST with body:

   ```fish
   peek cli -X POST -d '{"key":"value"}' example.com
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
       "requested_url": "example.com",
       "response": {
         /* HttpResponse object */
       }
     },
     { "requested_url": "bad.example", "error": "timeout" }
   ]
   ```

   Notes about concurrency:

   If you'd like, I can add a few short end-to-end examples that run against a local test server (useful for CI).

## Architecture

The application is structured into several modules:

- `main.rs`: Application entry point and GUI setup
- `ui.rs`: User interface implementation using egui
- `http_client.rs`: HTTP request handling and response processing
- `network_utils.rs`: Network utilities for IP resolution and local network info

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
