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
