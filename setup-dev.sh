#!/bin/bash

# AIChat Local Development Setup Script
# This script sets up everything needed for local development

set -e

echo "================================================"
echo "AIChat Local Development Setup"
echo "================================================"
echo ""

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${YELLOW}➜${NC} $1"
}

# Detect OS
OS="Unknown"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="Linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macOS"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    OS="Windows"
fi

echo "Detected OS: $OS"
echo ""

# 1. Install Rust
echo "Step 1: Installing Rust..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    print_status "Rust already installed: $RUST_VERSION"
else
    print_info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    print_status "Rust installed successfully"
fi

# Update Rust to latest stable
print_info "Updating Rust to latest stable..."
rustup update stable
rustup default stable

echo ""

# 2. Install PostgreSQL (optional for authentication features)
echo "Step 2: PostgreSQL Installation (Optional - for authentication features)"
read -p "Do you want to install PostgreSQL for authentication features? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v psql &> /dev/null; then
        PSQL_VERSION=$(psql --version)
        print_status "PostgreSQL already installed: $PSQL_VERSION"
    else
        case $OS in
            "Linux")
                if command -v apt-get &> /dev/null; then
                    print_info "Installing PostgreSQL via apt..."
                    sudo apt-get update
                    sudo apt-get install -y postgresql postgresql-contrib
                elif command -v yum &> /dev/null; then
                    print_info "Installing PostgreSQL via yum..."
                    sudo yum install -y postgresql-server postgresql-contrib
                    sudo postgresql-setup initdb
                elif command -v pacman &> /dev/null; then
                    print_info "Installing PostgreSQL via pacman..."
                    sudo pacman -S postgresql
                else
                    print_error "Package manager not supported. Please install PostgreSQL manually."
                fi
                ;;
            "macOS")
                if command -v brew &> /dev/null; then
                    print_info "Installing PostgreSQL via Homebrew..."
                    brew install postgresql
                    brew services start postgresql
                else
                    print_error "Homebrew not found. Please install Homebrew first or install PostgreSQL manually."
                fi
                ;;
            "Windows")
                print_info "Please download and install PostgreSQL from: https://www.postgresql.org/download/windows/"
                ;;
        esac
    fi

    # Create database if PostgreSQL is available
    if command -v psql &> /dev/null; then
        print_info "Setting up AIChat databases..."

        # Start PostgreSQL service if not running
        case $OS in
            "Linux")
                sudo systemctl start postgresql 2>/dev/null || true
                ;;
            "macOS")
                brew services start postgresql 2>/dev/null || true
                ;;
        esac

        # Create databases
        createdb aichat 2>/dev/null || print_info "Database 'aichat' already exists"
        createdb aichat_test 2>/dev/null || print_info "Database 'aichat_test' already exists"

        print_status "PostgreSQL setup complete"
    fi
else
    print_info "Skipping PostgreSQL installation. Authentication features will be disabled."
fi

echo ""

# 3. Install additional development tools
echo "Step 3: Installing additional development tools..."

# Install cargo-watch for auto-reloading during development
if cargo install --list | grep -q "cargo-watch"; then
    print_status "cargo-watch already installed"
else
    print_info "Installing cargo-watch..."
    cargo install cargo-watch
    print_status "cargo-watch installed"
fi

# Install cargo-edit for managing dependencies
if cargo install --list | grep -q "cargo-edit"; then
    print_status "cargo-edit already installed"
else
    print_info "Installing cargo-edit..."
    cargo install cargo-edit
    print_status "cargo-edit installed"
fi

echo ""

# 4. Set up environment variables
echo "Step 4: Setting up environment variables..."

if [ ! -f .env ]; then
    print_info "Creating .env file from .env.example..."
    if [ -f .env.example ]; then
        cp .env.example .env
        print_status ".env file created"
        print_info "Please edit .env file and add your API keys and configuration"
    else
        print_info "Creating default .env file..."
        cat > .env << 'EOF'
# Database Configuration (Optional - for authentication)
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aichat

# JWT Configuration
JWT_SECRET=development-secret-key-change-in-production
JWT_EXPIRY_HOURS=24

# Test Database
TEST_DATABASE_URL=postgresql://postgres:postgres@localhost:5432/aichat_test

# API Keys for LLM Providers (Add your keys here)
# OPENAI_API_KEY=sk-...
# ANTHROPIC_API_KEY=sk-ant-...
# GOOGLE_API_KEY=...
EOF
        print_status ".env file created with defaults"
        print_info "Please edit .env file and add your API keys"
    fi
else
    print_status ".env file already exists"
fi

echo ""

# 5. Build the project
echo "Step 5: Building the project..."
print_info "Running cargo build..."
if cargo build; then
    print_status "Build successful!"
else
    print_error "Build failed. Please check the error messages above."
    exit 1
fi

echo ""

# 6. Run migrations if PostgreSQL is set up
if [ -n "$DATABASE_URL" ] && command -v psql &> /dev/null; then
    echo "Step 6: Running database migrations..."
    print_info "Applying database schema..."

    # Check if we can connect to the database
    if psql "$DATABASE_URL" -c "SELECT 1" &> /dev/null; then
        # The application will run migrations automatically on startup
        print_status "Database connection successful"
        print_info "Migrations will be applied when the application starts"
    else
        print_error "Could not connect to database. Please check your DATABASE_URL in .env"
    fi
else
    print_info "Skipping database setup (PostgreSQL not configured)"
fi

echo ""

# 7. Create helper scripts
echo "Step 7: Creating helper scripts..."

# Create run script
cat > run-dev.sh << 'EOF'
#!/bin/bash
# Development run script with auto-reload

echo "Starting AIChat in development mode with auto-reload..."
echo "Press Ctrl+C to stop"
echo ""

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Run with cargo-watch for auto-reload
cargo watch -x run
EOF
chmod +x run-dev.sh
print_status "Created run-dev.sh"

# Create test script
cat > run-tests.sh << 'EOF'
#!/bin/bash
# Run all tests

echo "Running AIChat tests..."
echo ""

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Run tests
cargo test

# Run auth tests if PostgreSQL is configured
if [ -n "$DATABASE_URL" ]; then
    echo ""
    echo "Running authentication tests..."
    cargo test auth::tests
fi
EOF
chmod +x run-tests.sh
print_status "Created run-tests.sh"

# Create server script
cat > run-server.sh << 'EOF'
#!/bin/bash
# Run AIChat in server mode

echo "Starting AIChat server..."
echo ""

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
fi

# Run in server mode
cargo run -- --serve
EOF
chmod +x run-server.sh
print_status "Created run-server.sh"

echo ""
echo "================================================"
echo "Setup Complete!"
echo "================================================"
echo ""
print_status "All dependencies installed successfully!"
echo ""
echo "Next steps:"
echo "1. Edit .env file and add your API keys"
echo "2. Run './run-dev.sh' for development with auto-reload"
echo "3. Run './run-server.sh' to start the web server"
echo "4. Run './run-tests.sh' to run tests"
echo ""
echo "Quick commands:"
echo "  cargo build           - Build the project"
echo "  cargo run             - Run the CLI"
echo "  cargo run -- --serve  - Start web server"
echo "  cargo test            - Run tests"
echo ""
echo "Server endpoints (when running):"
echo "  http://localhost:8000/              - Enhanced GUI"
echo "  http://localhost:8000/playground    - LLM Playground"
echo "  http://localhost:8000/arena         - LLM Arena"
echo ""
if [ -n "$DATABASE_URL" ]; then
    echo "Authentication endpoints:"
    echo "  POST http://localhost:8000/auth/register"
    echo "  POST http://localhost:8000/auth/login"
    echo ""
fi
echo "Happy coding! 🚀"