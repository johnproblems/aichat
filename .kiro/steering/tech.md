# Technology Stack

## Language & Framework
- **Rust 2021 Edition**: Primary language with focus on performance and safety
- **Tokio**: Async runtime for concurrent operations
- **Clap**: Command-line argument parsing with derive macros
- **Serde**: Serialization/deserialization for JSON and YAML configs

## Key Dependencies
- **HTTP Client**: `reqwest` with rustls-tls, JSON, multipart support
- **Terminal UI**: `reedline` for REPL, `crossterm` for terminal control
- **Syntax Highlighting**: `syntect` with onig regex and plist loading
- **Streaming**: `tokio-stream`, `reqwest-eventsource` for real-time responses
- **Embeddings/RAG**: `hnsw_rs` for vector search, `bm25` for text ranking
- **Web Server**: `hyper` with `http-body-util` for local server capabilities

## Build System
- **Cargo**: Standard Rust package manager and build tool
- **Release Profile**: Optimized with LTO, strip symbols, size optimization (`opt-level = "z"`)

## Common Commands

### Development
```bash
# Build the project
cargo build

# Run with arguments
cargo run -- --help

# Run tests
cargo test

# Check code without building
cargo check
```

### Release
```bash
# Build optimized release
cargo build --release

# Install locally
cargo install --path .
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check dependencies
cargo audit
```

## Architecture Notes
- Modular client system supporting 20+ LLM providers
- Plugin-based function calling system
- Streaming response handling for real-time interaction
- Cross-platform clipboard and terminal integration