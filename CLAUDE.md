# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Task Master AI Instructions
**Import Task Master's development workflow commands and guidelines, treat as if import is in the main CLAUDE.md file.**
@./.taskmaster/CLAUDE.md

## Project Overview

AIChat is an all-in-one LLM CLI tool written in Rust that provides a unified interface for interacting with 20+ LLM providers including OpenAI, Claude, Gemini, Ollama, and more. The project features multiple interaction modes (CMD, REPL, Server), advanced capabilities like RAG, function calling, AI agents, and a built-in web server with playground and arena interfaces.

## Essential Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Run with arguments (development)
cargo run -- --help

# Build optimized release
cargo build --release

# Install locally from source
cargo install --path .

# Run single test
cargo test <test_name>

# Run tests
cargo test
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint code 
cargo clippy

# Check code without building
cargo check
```

### Local Server Mode
```bash
# Start local server (default port 8000)
cargo run -- --serve

# Start on specific port
cargo run -- --serve 3000

# Start on specific IP/port
cargo run -- --serve 192.168.1.100:8080
```

## Architecture Overview

### Core Components

- **CLI Interface** (`src/cli.rs`): Command-line argument parsing using Clap with extensive options for models, sessions, roles, agents, RAG, and server mode
- **Client System** (`src/client/`): Modular provider system supporting 20+ LLM providers with unified interface, streaming support, and provider-specific implementations
- **Configuration** (`src/config/`): Comprehensive config management including roles, sessions, agents, and input processing
- **Server** (`src/serve.rs`): Built-in HTTP server providing Chat Completions API, Embeddings API, Rerank API, and web interfaces (playground, arena, enhanced GUI)
- **REPL** (`src/repl/`): Interactive chat interface with syntax highlighting, completion, and multi-line support
- **RAG System** (`src/rag/`): Retrieval-Augmented Generation with vector search, document processing, and intelligent chunking
- **Function Calling** (`src/function.rs`): Tool integration system for connecting LLMs to external APIs and services

### Working Modes

The application operates in three primary modes determined by command-line arguments:

1. **CMD Mode**: Single command execution with immediate output
2. **REPL Mode**: Interactive chat session with persistent context
3. **Server Mode**: HTTP server providing APIs and web interfaces

### Key Architectural Patterns

- **Provider Abstraction**: All LLM providers implement common traits for unified interaction
- **Streaming Support**: Real-time response streaming across all providers and interfaces
- **Configuration Management**: YAML-based configuration with role and session persistence
- **Asset Embedding**: Web assets are embedded using `rust-embed` for single-binary distribution
- **Cross-Platform Support**: Conditional compilation for platform-specific features (clipboard, terminal)

## Project Structure

### Source Code Organization (`src/`)
- `main.rs` - Entry point, mode detection, and orchestration
- `cli.rs` - Command-line interface definitions with Clap
- `serve.rs` - HTTP server with APIs and web interfaces
- `client/` - Multi-provider LLM client system
  - `mod.rs` - Provider registration and common interfaces
  - `{provider}.rs` - Individual provider implementations (openai.rs, claude.rs, etc.)
  - `stream.rs` - Streaming response handling
  - `message.rs` - Message formatting and processing
- `config/` - Configuration management
  - `mod.rs` - Main configuration loading and management
  - `role.rs` - AI role system
  - `session.rs` - Conversation session persistence
  - `agent.rs` - AI agent configuration and management
- `repl/` - Interactive REPL implementation
- `render/` - Response formatting and syntax highlighting
- `rag/` - Retrieval-Augmented Generation system
- `utils/` - Shared utilities and helpers

### Key Files
- `Cargo.toml` - Dependencies and build configuration
- `models.yaml` - LLM model configurations and capabilities
- `config.example.yaml` - Example user configuration
- `assets/` - Embedded web UI files (HTML, CSS, JS)
- `.kiro/` - Project specifications and architecture documentation

## Special Features

### Web Server Capabilities
The built-in server (`--serve` flag) provides:
- **Chat Completions API**: OpenAI-compatible API endpoint at `/v1/chat/completions`
- **Embeddings API**: Text embedding endpoint at `/v1/embeddings`  
- **Rerank API**: Document reranking at `/v1/rerank`
- **LLM Playground**: Web interface for model interaction
- **LLM Arena**: Side-by-side model comparison tool
- **Enhanced GUI**: Matrix-themed web interface (as per .kiro specs)

### Multi-Provider Support
Unified interface supporting:
- OpenAI (GPT models)
- Claude (Anthropic)
- Gemini (Google AI Studio)
- Azure OpenAI
- AWS Bedrock
- Ollama (local models)
- Plus 15+ other providers

### Advanced Features
- **RAG Integration**: Document ingestion with vector search
- **Function Calling**: External tool integration
- **AI Agents**: Custom agents with instructions, tools, and documents
- **Session Management**: Persistent conversation contexts
- **Role System**: Customizable AI behavior patterns
- **Shell Assistant**: Natural language to shell command conversion

## Configuration

### User Configuration
- Default location: `~/.config/aichat/config.yaml`
- Model configurations: `models.yaml` in repo root
- Environment variables: Support for API keys and settings
- Agent configurations: Individual YAML files for each agent

### API Key Management
Supports API keys for all providers via environment variables:
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY` 
- `GOOGLE_API_KEY`
- And provider-specific keys

## Development Notes

### Dependencies
- **Tokio**: Async runtime for concurrent operations
- **Hyper**: HTTP server implementation
- **Clap**: Command-line parsing with derive macros
- **Serde**: JSON/YAML serialization
- **Reqwest**: HTTP client with streaming support
- **Reedline**: Advanced REPL functionality
- **Syntect**: Syntax highlighting for code blocks

### Testing
- Unit tests throughout codebase
- Integration tests for major features
- Provider-specific test suites

### Future Development (per .kiro specs)
The project has specifications for transformation into a comprehensive web-based platform with:
- Matrix-themed web GUI
- API key marketplace
- Authentication system
- Document processing
- Cloud integrations
- Peer-to-peer billing system

These specifications are detailed in `.kiro/specs/web-gui-transformation/` and should be referenced for any web interface enhancements.