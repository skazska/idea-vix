#!/bin/bash

# Integration Test Runner Script
# Usage: ./test_integration.sh [--smoke]

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
SMOKE_ONLY=false
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVER_DIR="$(dirname $SCRIPT_DIR)/server/ws"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --smoke)
            SMOKE_ONLY=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--smoke]"
            echo "  --smoke    Run only smoke tests"
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

print_status "Starting Integration Test Suite"
if [[ "$SMOKE_ONLY" == "true" ]]; then
    print_status "Running SMOKE tests only"
fi

# Step 1: Check dependencies
print_status "Checking dependencies..."
command -v cargo >/dev/null 2>&1 || { print_error "cargo is required but not installed. Aborting."; exit 1; }

# Step 2: Set up environment for in-memory database
export RUST_LOG="warn"  # Reduce noise for integration tests

print_status "Environment configured for integration tests"

# Step 3: Navigate to server directory
cd "$SERVER_DIR"

# Step 4: Run integration tests
print_status "Running integration tests..."

if [[ "$SMOKE_ONLY" == "true" ]]; then
    # Run only smoke tests (tests with "smoke" in the name)
    print_status "Running smoke integration tests..."
    cargo test --test "*smoke*" -- --test-threads=1
    
    # Also run specific smoke test functions
    print_status "Running individual smoke test functions..."
    cargo test smoke -- --test-threads=1
else
    # Run all integration tests (tests in the tests/ directory)
    print_status "Running all integration tests..."
    cargo test --tests -- --test-threads=1
fi

TEST_EXIT_CODE=$?

if [[ $TEST_EXIT_CODE -eq 0 ]]; then
    print_status "✅ Integration tests completed successfully"
else
    print_error "❌ Integration tests failed with exit code $TEST_EXIT_CODE"
fi

exit $TEST_EXIT_CODE