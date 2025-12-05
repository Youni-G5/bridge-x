#!/bin/bash
# Run BridgeX locally for development

set -e

echo "🚀 Starting BridgeX locally..."
echo ""

# Function to cleanup background processes
cleanup() {
    echo ""
    echo "🛑 Stopping services..."
    kill $(jobs -p) 2>/dev/null || true
    exit
}

trap cleanup SIGINT SIGTERM

# Start backend
echo "📡 Starting backend server..."
cd backend
cargo run &
BACKEND_PID=$!
echo "   Backend PID: $BACKEND_PID"
cd ..

# Wait for backend to start
echo "⏳ Waiting for backend..."
sleep 3

# Check backend health
if curl -s http://127.0.0.1:8080/api/v1/health > /dev/null; then
    echo "✅ Backend is running"
else
    echo "❌ Backend failed to start"
    cleanup
    exit 1
fi
echo ""

# Start desktop (optional)
read -p "Start desktop app? (y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🖥️  Starting desktop app..."
    cd desktop
    cargo tauri dev &
    DESKTOP_PID=$!
    echo "   Desktop PID: $DESKTOP_PID"
    cd ..
fi
echo ""

# Start mobile (optional)
if command -v flutter &> /dev/null; then
    read -p "Start mobile app? (y/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "📱 Starting mobile app..."
        cd mobile
        flutter run &
        MOBILE_PID=$!
        echo "   Mobile PID: $MOBILE_PID"
        cd ..
    fi
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ BridgeX is running!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📍 Endpoints:"
echo "   Backend:  http://127.0.0.1:8080"
echo "   Health:   http://127.0.0.1:8080/api/v1/health"
echo "   Status:   http://127.0.0.1:8080/api/v1/status"
echo ""
echo "Press Ctrl+C to stop all services"
echo ""

# Wait for user interrupt
wait
