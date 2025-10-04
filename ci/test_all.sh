#!/bin/bash

# Full Test Suite Runner Script
# Usage: ./test_all.sh [--smoke]

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
SMOKE_ONLY=false
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --smoke)
            SMOKE_ONLY=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--smoke]"
            echo "Run the full test suite: unit tests, integration tests, and E2E tests"
            echo "  --smoke    Run only smoke tests for integration and E2E"
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

print_section() {
    echo -e "${BLUE}[SECTION]${NC} $1"
    echo "=================================="
}

print_section "Starting Full Test Suite"
if [[ "$SMOKE_ONLY" == "true" ]]; then
    print_status "Running SMOKE tests only"
fi

# Track overall results
UNIT_RESULT=0
INTEGRATION_RESULT=0
E2E_RESULT=0

# Step 1: Run Unit Tests
print_section "Running Unit Tests"
if ./test_unit.sh; then
    print_status "✅ Unit tests passed"
    UNIT_RESULT=0
else
    print_error "❌ Unit tests failed"
    UNIT_RESULT=1
fi

# Step 2: Run Integration Tests
print_section "Running Integration Tests"
if [[ "$SMOKE_ONLY" == "true" ]]; then
    if ./test_integration.sh --smoke; then
        print_status "✅ Integration smoke tests passed"
        INTEGRATION_RESULT=0
    else
        print_error "❌ Integration smoke tests failed"
        INTEGRATION_RESULT=1
    fi
else
    if ./test_integration.sh; then
        print_status "✅ Integration tests passed"
        INTEGRATION_RESULT=0
    else
        print_error "❌ Integration tests failed"
        INTEGRATION_RESULT=1
    fi
fi

# Step 3: Run E2E Tests
print_section "Running E2E Tests"
if [[ "$SMOKE_ONLY" == "true" ]]; then
    if ./test_e2e.sh --smoke; then
        print_status "✅ E2E smoke tests passed"
        E2E_RESULT=0
    else
        print_error "❌ E2E smoke tests failed"
        E2E_RESULT=1
    fi
else
    if ./test_e2e.sh; then
        print_status "✅ E2E tests passed"
        E2E_RESULT=0
    else
        print_error "❌ E2E tests failed"
        E2E_RESULT=1
    fi
fi

# Step 4: Summary
print_section "Test Suite Summary"

echo -n "Unit Tests: "
if [[ $UNIT_RESULT -eq 0 ]]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC}"
fi

echo -n "Integration Tests: "
if [[ $INTEGRATION_RESULT -eq 0 ]]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC}"
fi

echo -n "E2E Tests: "
if [[ $E2E_RESULT -eq 0 ]]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC}"
fi

# Calculate overall result
TOTAL_FAILURES=$((UNIT_RESULT + INTEGRATION_RESULT + E2E_RESULT))

if [[ $TOTAL_FAILURES -eq 0 ]]; then
    print_status "🎉 All tests passed successfully!"
    exit 0
else
    print_error "❌ $TOTAL_FAILURES test suite(s) failed"
    exit 1
fi