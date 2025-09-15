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
