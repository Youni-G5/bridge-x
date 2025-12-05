#!/bin/bash
# BridgeX Bootstrap Script
# Installs all required dependencies for development

set -e

echo "🚀 BridgeX Bootstrap Starting..."
echo ""

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    OS="windows"
else
    echo "❌ Unsupported OS: $OSTYPE"
    exit 1
fi

echo "📍 Detected OS: $OS"
echo ""

# Check Rust
echo "🔍 Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust installed: $RUST_VERSION"
else
    echo "❌ Rust not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust installed successfully"
fi
echo ""

# Check Flutter
echo "🔍 Checking Flutter installation..."
if command -v flutter &> /dev/null; then
    FLUTTER_VERSION=$(flutter --version | head -n 1)
    echo "✅ Flutter installed: $FLUTTER_VERSION"
else
    echo "⚠️  Flutter not found. Please install manually:"
    echo "   https://flutter.dev/docs/get-started/install"
fi
echo ""

# Install platform-specific dependencies
if [[ "$OS" == "linux" ]]; then
    echo "📦 Installing Linux dependencies..."
    sudo apt-get update
    sudo apt-get install -y \
        build-essential \
        curl \
        wget \
        file \
        libssl-dev \
        pkg-config \
        libgtk-3-dev \
        libwebkit2gtk-4.0-dev \
        libappindicator3-dev \
        librsvg2-dev \
        patchelf \
        sqlite3 \
        libsqlite3-dev
    echo "✅ Linux dependencies installed"
elif [[ "$OS" == "macos" ]]; then
    echo "📦 Checking macOS dependencies..."
    if ! command -v brew &> /dev/null; then
        echo "❌ Homebrew not found. Installing..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    echo "✅ Homebrew available"
fi
echo ""

# Install Tauri CLI
echo "🔨 Installing Tauri CLI..."
if cargo install --list | grep -q "tauri-cli"; then
    echo "✅ Tauri CLI already installed"
else
    cargo install tauri-cli --version '^2.0.0-beta'
    echo "✅ Tauri CLI installed"
fi
echo ""

# Install cargo tools
echo "🔨 Installing Rust development tools..."
cargo install cargo-audit 2>/dev/null || echo "⚠️  cargo-audit already installed"
cargo install cargo-watch 2>/dev/null || echo "⚠️  cargo-watch already installed"
echo "✅ Rust tools ready"
echo ""

# Setup backend
echo "🏗️  Setting up backend..."
cd backend
cargo fetch
echo "✅ Backend dependencies fetched"
cd ..
echo ""

# Setup mobile
if command -v flutter &> /dev/null; then
    echo "🏗️  Setting up mobile app..."
    cd mobile
    flutter pub get
    echo "✅ Mobile dependencies installed"
    cd ..
else
    echo "⚠️  Skipping mobile setup (Flutter not installed)"
fi
echo ""

# Create data directories
echo "📁 Creating data directories..."
mkdir -p data logs
echo "✅ Directories created"
echo ""

# Summary
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✨ Bootstrap Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📋 Next Steps:"
echo "   1. Run backend:  cd backend && cargo run"
echo "   2. Run desktop:  cd desktop && cargo tauri dev"
echo "   3. Run mobile:   cd mobile && flutter run"
echo ""
echo "📚 Documentation: docs/"
echo "🐛 Issues: https://github.com/Youni-G5/bridge-x/issues"
echo ""
