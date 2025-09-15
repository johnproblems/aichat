# AIChat Quick Start Guide

## Prerequisites

Before building AIChat, you need to install the following system dependencies:

### Required Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev
```

#### Linux (Fedora/RHEL)
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config openssl-devel
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Or use Homebrew
brew install pkg-config
```

#### Windows
Install Visual Studio Build Tools or full Visual Studio with C++ development workload.

### Rust Installation

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env
```

## Quick Setup

### 1. Clone and Enter Directory
```bash
git clone https://github.com/sigoden/aichat.git
cd aichat
```

### 2. Set Up Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env and add at least one API key:
# OPENAI_API_KEY=sk-...
# or ANTHROPIC_API_KEY=sk-ant-...
# or GOOGLE_API_KEY=...
```

### 3. Build the Project
```bash
# Build (debug mode - faster compilation)
cargo build

# Or build optimized version (slower compilation, faster runtime)
cargo build --release
```

### 4. Run AIChat

#### CLI Mode
```bash
# Run in development mode
cargo run

# Or run the release build
./target/release/aichat
```

#### Server Mode
```bash
# Start web server on port 8000
cargo run -- --serve

# Or specific port
cargo run -- --serve 3000
```

## Available Interfaces

Once the server is running (port 8000 by default):

- **Enhanced GUI**: http://localhost:8000/
- **LLM Playground**: http://localhost:8000/playground
- **LLM Arena**: http://localhost:8000/arena

## API Endpoints

- **Chat Completions**: `POST http://localhost:8000/v1/chat/completions`
- **Embeddings**: `POST http://localhost:8000/v1/embeddings`
- **Rerank**: `POST http://localhost:8000/v1/rerank`

## Optional: PostgreSQL for Authentication

If you want to enable user authentication and management features:

### Install PostgreSQL
```bash
# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql
brew services start postgresql

# Create database
createdb aichat
```

### Configure Database
Add to your `.env`:
```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aichat
JWT_SECRET=your-secret-key-here
```

The database schema will be created automatically when you first run the server.

## Testing Your Setup

### Test CLI
```bash
echo "Hello, how are you?" | cargo run
```

### Test Server
```bash
# Start server
cargo run -- --serve &

# Test API
curl -X POST http://localhost:8000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "default",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

## Troubleshooting

### Build Errors

**"linker `cc` not found"**
- Install build-essential (Linux) or Xcode tools (macOS)

**"Could not find OpenSSL"**
- Install libssl-dev (Linux) or openssl (macOS)

### Runtime Errors

**"No API keys configured"**
- Add at least one LLM provider API key to your `.env` file

**"Database connection failed"**
- Check PostgreSQL is running
- Verify DATABASE_URL in `.env`

## Next Steps

1. **Configure API Keys**: Edit `.env` and add your provider keys
2. **Explore Features**: Try different models and features
3. **Read Documentation**: See `README.md` for full documentation
4. **Customize**: Modify `config.yaml` for advanced settings

## Development Tips

### Watch Mode (Auto-reload)
```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Getting Help

- **Documentation**: See `/docs` folder
- **Issues**: https://github.com/sigoden/aichat/issues
- **Examples**: Check `/examples` folder

## Minimal Working Example

1. **Install Rust and build tools**
2. **Set one API key in `.env`**
3. **Run `cargo build`**
4. **Run `cargo run`**

That's it! You should now have AIChat running locally.