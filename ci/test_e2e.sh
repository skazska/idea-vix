#!/bin/bash

# E2E Test Runner Script
# Usage: ./test_e2e.sh [--smoke]

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
FRONTEND_DIR="$(dirname $SCRIPT_DIR)/ui/web"

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

# Function to cleanup background processes
cleanup() {
    print_status "Cleaning up..."
    if [[ -n "$BACKEND_PID" ]]; then
        print_status "Stopping backend server (PID: $BACKEND_PID)"
        kill $BACKEND_PID 2>/dev/null || true
        wait $BACKEND_PID 2>/dev/null || true
    fi
    if [[ -n "$FRONTEND_PID" ]]; then
        print_status "Stopping frontend server (PID: $FRONTEND_PID)"
        kill $FRONTEND_PID 2>/dev/null || true
        wait $FRONTEND_PID 2>/dev/null || true
    fi

    sudo kill -9 $(sudo fuser 4173/tcp)
}

# Set trap for cleanup on script exit
trap cleanup EXIT

print_status "Starting E2E Test Suite"
if [[ "$SMOKE_ONLY" == "true" ]]; then
    print_status "Running SMOKE tests only"
fi

# Step 1: Check dependencies
print_status "Checking dependencies..."
command -v cargo >/dev/null 2>&1 || { print_error "cargo is required but not installed. Aborting."; exit 1; }
command -v npm >/dev/null 2>&1 || { print_error "npm is required but not installed. Aborting."; exit 1; }
command -v sqlx >/dev/null 2>&1 || { print_error "sqlx-cli is required but not installed. Run: cargo install sqlx-cli"; exit 1; }

# Step 2: Set up environment variables for in-memory database
# export WS_DATABASE_URL="file:e2e_test.db?mode=memory&cache=shared"
export WS_DATABASE_URL="sqlite::memory:?cache=shared"
export WS_PORT=7879
export RUST_LOG="info"
export UI_BASE_URL="http://localhost:4173"

print_status "Database URL: $WS_DATABASE_URL"
print_status "Backend Port: $WS_PORT"

# Step 3: Navigate to server directory and start backend
cd "$SERVER_DIR"
print_status "Setting up in-memory database and running migrations..."

print_status "Starting backend server on port $WS_PORT..."
cargo run --bin=ws -- --init-db > backend.log 2>&1 &
BACKEND_PID=$!
print_status "Backend PID: $BACKEND_PID"

# Check if backend is still running
if ! kill -0 $BACKEND_PID 2>/dev/null; then
    print_error "Backend server failed to start. Check backend.log for details:"
    cat backend.log
    exit 1
fi

# Step 5: Navigate to frontend and run E2E tests
cd "$FRONTEND_DIR"
print_status "Setting up frontend dependencies..."

# Install frontend dependencies if needed
if [[ ! -d "node_modules" ]]; then
    print_status "Installing frontend dependencies..."
    npm install
else
    print_status "Frontend dependencies already installed"
fi

npm run build
npm run preview > frontend.log 2>&1 &
FRONTEND_PID=$!
print_status "Frontend PID: $FRONTEND_PID"

# Check if frontend is still running
if ! kill -0 $FRONTEND_PID 2>/dev/null; then
    print_error "Frontend server failed to start. Check frontend.log for details:"
    cat frontend.log
    exit 1
fi


# Install Playwright browsers if needed
# print_status "Ensuring Playwright browsers are installed..."
# npx playwright install --with-deps


# Wait for backend to be ready
print_status "Waiting for backend to be ready..."
sleep 5

# Test if backend is responding
for i in {1..10}; do
    if curl -s http://localhost:$WS_PORT/health >/dev/null 2>&1; then
        print_status "Backend is ready and responding"
        break
    elif [[ $i -eq 10 ]]; then
        print_error "Backend did not become ready after 10 attempts"
        print_error "Backend log:"
        cat backend.log
        exit 1
    else
        print_status "Waiting for backend... attempt $i/10"
        sleep 5
    fi
done


# Test if frontend is responding
for j in {1..10}; do
    if curl -s "$UI_BASE_URL" >/dev/null 2>&1; then
        print_status "Frontend is ready and responding"
        break
    elif [[ $j -eq 10 ]]; then
        print_error "Frontend did not become ready after 10 attempts"
        print_error "Frontend log:"
        cat frontend.log
        exit 1
    else
        print_status "Waiting for frontend... attempt $j/10"
        sleep 5
    fi
done

# Step 6: Run E2E tests
print_status "Running E2E tests..."

if [[ "$SMOKE_ONLY" == "true" ]]; then
    # Run tests matching smoke pattern
    npm run test:e2e:smoke:headed
else
    # Run all E2E tests
    npm run test:e2e
fi

TEST_EXIT_CODE=$?

if [[ $TEST_EXIT_CODE -eq 0 ]]; then
    print_status "✅ E2E tests completed successfully"
else
    print_error "❌ E2E tests failed with exit code $TEST_EXIT_CODE"
fi

# Cleanup will happen automatically via trap
exit $TEST_EXIT_CODE