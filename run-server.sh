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
