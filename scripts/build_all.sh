#!/bin/bash
# Build all BridgeX components

set -e

echo "🏗️  Building BridgeX..."
echo ""

# Build backend
echo "📦 Building backend..."
cd backend
cargo build --release
echo "✅ Backend built: target/release/bridgex-server"
cd ..
echo ""

# Build desktop
echo "🖥️  Building desktop app..."
cd desktop
cargo tauri build
echo "✅ Desktop app built"
cd ..
echo ""
# Build mobile (if Flutter is available)
if command -v flutter &> /dev/null; then
    echo "📱 Building mobile app..."
    cd mobile
    
    # Android
    echo "  Building Android APK..."
    flutter build apk --release
    echo "  ✅ APK: mobile/build/app/outputs/flutter-apk/app-release.apk"
    
    cd ..
else
    echo "⚠️  Skipping mobile build (Flutter not installed)"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ Build Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📦 Build Artifacts:"
echo "   Backend:  backend/target/release/bridgex-server"
echo "   Desktop:  desktop/src-tauri/target/release/"
if command -v flutter &> /dev/null; then
    echo "   Mobile:   mobile/build/app/outputs/flutter-apk/"
fi
echo ""
