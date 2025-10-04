#!/bin/bash

# Unit Test Runner Script
# Usage: ./test_unit.sh

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_DIR="$(dirname $SCRIPT_DIR)/server/ws"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "Usage: $0"
            echo "Run unit tests for the backend"
            echo "  --help     Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_status "Starting Unit Test Suite"

# Step 1: Check dependencies
print_status "Checking dependencies..."
command -v cargo >/dev/null 2>&1 || { print_error "cargo is required but not installed. Aborting."; exit 1; }

# Step 2: Set up environment
export RUST_LOG="warn"  # Reduce noise for unit tests

print_status "Environment configured for unit tests"

# Step 3: Navigate to server directory
cd "$SERVER_DIR"

# Step 4: Run unit tests
print_status "Running unit tests..."

# Run only unit tests (lib tests, not integration tests in tests/ directory)
cargo test --lib --bins

TEST_EXIT_CODE=$?

if [[ $TEST_EXIT_CODE -eq 0 ]]; then
    print_status "✅ Unit tests completed successfully"
else
    print_error "❌ Unit tests failed with exit code $TEST_EXIT_CODE"
fi

exit $TEST_EXIT_CODE