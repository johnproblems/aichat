# Project Structure

## Root Directory
- `Cargo.toml` - Package manifest with dependencies and metadata
- `Cargo.lock` - Dependency lock file
- `README.md` - Project documentation and usage guide
- `LICENSE-MIT` / `LICENSE-APACHE` - Dual licensing
- `models.yaml` - LLM model configurations
- `config.example.yaml` - Example configuration file

## Source Code (`src/`)

### Core Modules
- `main.rs` - Application entry point and orchestration
- `cli.rs` - Command-line interface definitions using Clap
- `serve.rs` - HTTP server implementation for API and web UI

### Client System (`src/client/`)
- `mod.rs` - Client registration and provider definitions
- `common.rs` - Shared client functionality
- `message.rs` - Message handling and formatting
- `model.rs` - Model definitions and capabilities
- `stream.rs` - Streaming response handling
- Provider-specific clients: `openai.rs`, `claude.rs`, `gemini.rs`, etc.

### Configuration (`src/config/`)
- `mod.rs` - Main configuration management
- `role.rs` - Role system implementation
- `session.rs` - Session persistence and management
- `agent.rs` - AI agent configuration
- `input.rs` - Input processing and validation

### Features
- `function.rs` - Function calling and tool integration
- `src/rag/` - Retrieval-Augmented Generation system
- `src/render/` - Output formatting and syntax highlighting
- `src/repl/` - Interactive REPL implementation
- `src/utils/` - Shared utilities and helpers

## Assets (`assets/`)
- `*.html` - Web UI templates (playground, arena, demo)
- `*.css` - Styling and themes
- `*.js` - Client-side functionality
- `syntaxes.bin` / `*.theme.bin` - Syntax highlighting data
- `roles/` - Built-in role definitions

## Scripts (`scripts/`)
- `completions/` - Shell completion scripts for bash, zsh, fish, etc.
- `shell-integration/` - Shell integration helpers

## Conventions
- Use `mod.rs` for module organization
- Group related functionality in subdirectories
- Separate client implementations by provider
- Configuration files use YAML format
- Assets are embedded using `rust-embed`
- Cross-platform compatibility maintained throughout