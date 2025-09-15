# Local Development Setup Guide

## Quick Start

Run the automated setup script:

```bash
./setup-dev.sh
```

This will install all dependencies and set up your development environment.

## Manual Setup

### 1. Install Rust

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Install PostgreSQL (Optional - for authentication)

#### macOS
```bash
brew install postgresql
brew services start postgresql
```

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

#### Fedora/RHEL
```bash
sudo dnf install postgresql postgresql-server
sudo postgresql-setup --initdb
sudo systemctl start postgresql
```

#### Windows
Download from: https://www.postgresql.org/download/windows/

### 3. Set Up Databases

```bash
# Create main database
createdb aichat

# Create test database
createdb aichat_test
```

### 4. Configure Environment

Create a `.env` file in the project root:

```bash
# Copy example file
cp .env.example .env

# Edit with your configuration
nano .env
```

Essential configuration:

```env
# For authentication features (optional)
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aichat
JWT_SECRET=your-development-secret-key

# Add at least one LLM provider API key
OPENAI_API_KEY=sk-...
# or
ANTHROPIC_API_KEY=sk-ant-...
# or
GOOGLE_API_KEY=...
```

### 5. Build the Project

```bash
# Download dependencies and build
cargo build

# Build with optimizations (slower but faster runtime)
cargo build --release
```

## Development Commands

### Running the Application

```bash
# CLI mode (interactive chat)
cargo run

# Start web server
cargo run -- --serve

# Server on specific port
cargo run -- --serve 3000

# With specific model
cargo run -- -m gpt-4

# With system role
cargo run -- -r coder
```

### Development with Auto-Reload

Install cargo-watch:
```bash
cargo install cargo-watch
```

Run with auto-reload:
```bash
# Auto-reload on file changes
cargo watch -x run

# Auto-reload server
cargo watch -x "run -- --serve"
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test auth::tests

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_user_registration
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy

# Check without building
cargo check
```

## Project Structure

```
aichat/
├── src/
│   ├── main.rs           # Entry point
│   ├── auth/             # Authentication system
│   │   ├── mod.rs        # Auth service
│   │   ├── middleware.rs # JWT middleware
│   │   ├── routes.rs     # Auth endpoints
│   │   └── tests.rs      # Auth tests
│   ├── client/           # LLM clients
│   ├── config/           # Configuration
│   ├── database.rs       # Database connection
│   ├── serve.rs          # HTTP server
│   └── utils/            # Utilities
├── migrations/           # Database migrations
├── assets/              # Web UI assets
├── docs/                # Documentation
├── Cargo.toml           # Dependencies
├── .env                 # Local configuration
└── setup-dev.sh        # Setup script
```

## Available Features

### Without Database (Basic Mode)
- ✅ All LLM providers (20+)
- ✅ CLI chat interface
- ✅ Web playground
- ✅ API endpoints
- ✅ RAG system
- ✅ Function calling

### With Database (Full Mode)
All basic features plus:
- ✅ User authentication
- ✅ Session management
- ✅ User profiles
- ✅ Chat history
- ✅ API key marketplace (coming)

## Troubleshooting

### Rust Installation Issues

If rustup fails:
```bash
# Remove old installation
rm -rf ~/.cargo ~/.rustup

# Reinstall
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### PostgreSQL Connection Issues

Check PostgreSQL is running:
```bash
# macOS
brew services list | grep postgresql

# Linux
systemctl status postgresql

# Test connection
psql -U postgres -d aichat -c "SELECT 1"
```

Fix permission issues:
```bash
# Edit pg_hba.conf
sudo nano /etc/postgresql/*/main/pg_hba.conf

# Change authentication to trust for local
# local   all   all   trust

# Restart PostgreSQL
sudo systemctl restart postgresql
```

### Build Errors

Clear cache and rebuild:
```bash
cargo clean
cargo build
```

Update dependencies:
```bash
cargo update
```

### Missing System Dependencies

Ubuntu/Debian:
```bash
sudo apt install build-essential pkg-config libssl-dev
```

macOS:
```bash
xcode-select --install
```

## Development Tips

### 1. Use Environment Variables

```bash
# Set temporarily
export OPENAI_API_KEY=sk-...

# Or use .env file
echo "OPENAI_API_KEY=sk-..." >> .env
```

### 2. Debug Output

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Specific module
RUST_LOG=aichat::auth=debug cargo run
```

### 3. Performance Profiling

```bash
# Build with debug symbols
cargo build --release

# Profile with perf (Linux)
perf record --call-graph=dwarf cargo run --release
perf report
```

### 4. Database Migrations

```bash
# Apply migrations manually
psql $DATABASE_URL < migrations/001_initial_schema.sql

# Check database schema
psql $DATABASE_URL -c "\dt"
```

## Helper Scripts

The setup creates these helper scripts:

- `./run-dev.sh` - Run with auto-reload
- `./run-server.sh` - Start web server
- `./run-tests.sh` - Run test suite

## API Testing

### Using curl

```bash
# Test chat completion
curl -X POST http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "default",
    "messages": [{"role": "user", "content": "Hello"}]
  }'

# Test authentication
curl -X POST http://localhost:8000/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "username": "testuser",
    "password": "Test123!@#"
  }'
```

### Using HTTPie

```bash
# Install HTTPie
pip install httpie

# Test endpoints
http POST localhost:8000/v1/chat/completions \
  model=default \
  messages:='[{"role": "user", "content": "Hello"}]'
```

## VS Code Setup

Create `.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

Create `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug CLI",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/aichat",
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "name": "Debug Server",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceFolder}/target/debug/aichat",
      "args": ["--serve"],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## Next Steps

1. **Configure API Keys**: Edit `.env` and add your LLM provider API keys
2. **Run the Application**: Use `cargo run` or the helper scripts
3. **Explore Features**: Try different modes and endpoints
4. **Contribute**: Make improvements and submit PRs

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [AIChat Documentation](./README.md)

## Support

- GitHub Issues: Report bugs and request features
- Documentation: Check `/docs` folder
- Examples: See `/examples` folder

Happy coding! 🦀