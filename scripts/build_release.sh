#!/bin/bash

set -e  # Exit on error

echo "====================================="
echo "  BridgeX Release Builder"
echo "====================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Get version from Cargo.toml or git tag
VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "0.1.0")
echo -e "${GREEN}Version: $VERSION${NC}"
echo ""

# Create release directory
RELEASE_DIR="releases/$VERSION"
mkdir -p "$RELEASE_DIR"
echo -e "${GREEN}Release directory: $RELEASE_DIR${NC}"
echo ""

# Function to build backend
build_backend() {
    echo -e "${YELLOW}[1/4] Building backend server...${NC}"
    cd backend
    cargo build --release
    cd ..
    echo -e "${GREEN}✓ Backend built successfully${NC}"
    echo ""
}

# Function to build desktop app
build_desktop() {
    echo -e "${YELLOW}[2/4] Building desktop app...${NC}"
    
    # Check if Tauri CLI is installed
    if ! command -v cargo-tauri &> /dev/null; then
        echo -e "${RED}Tauri CLI not found. Installing...${NC}"
        cargo install tauri-cli --version '^2.0.0-beta'
    fi
    
    cd desktop
    
    # Copy backend binary to resources
    echo "Bundling backend with desktop app..."
    mkdir -p src-tauri/resources
    
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        cp ../backend/target/release/bridgex-server.exe src-tauri/resources/
    else
        cp ../backend/target/release/bridgex-server src-tauri/resources/
    fi
    
    # Build Tauri app
    cargo tauri build --release
    
    cd ..
    echo -e "${GREEN}✓ Desktop app built successfully${NC}"
    echo ""
}

# Function to build mobile app
build_mobile() {
    echo -e "${YELLOW}[3/4] Building mobile app (Android)...${NC}"
    
    # Check if Flutter is installed
    if ! command -v flutter &> /dev/null; then
        echo -e "${RED}Flutter not found. Please install Flutter first.${NC}"
        exit 1
    fi
    
    cd mobile
    
    # Get dependencies
    flutter pub get
    
    # Build APK
    flutter build apk --release
    
    cd ..
    echo -e "${GREEN}✓ Mobile app built successfully${NC}"
    echo ""
}

# Function to collect artifacts
collect_artifacts() {
    echo -e "${YELLOW}[4/4] Collecting release artifacts...${NC}"
    
    # Detect platform
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        PLATFORM="linux"
        DESKTOP_BUNDLE="desktop/src-tauri/target/release/bundle"
        
        # Copy AppImage
        if [ -f "$DESKTOP_BUNDLE/appimage/"*.AppImage ]; then
            cp "$DESKTOP_BUNDLE/appimage/"*.AppImage "$RELEASE_DIR/BridgeX-$VERSION.AppImage"
            echo -e "${GREEN}✓ Copied AppImage${NC}"
        fi
        
        # Copy DEB
        if [ -f "$DESKTOP_BUNDLE/deb/"*.deb ]; then
            cp "$DESKTOP_BUNDLE/deb/"*.deb "$RELEASE_DIR/bridgex-$VERSION.deb"
            echo -e "${GREEN}✓ Copied DEB package${NC}"
        fi
        
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        PLATFORM="macos"
        DESKTOP_BUNDLE="desktop/src-tauri/target/release/bundle"
        
        # Copy DMG
        if [ -f "$DESKTOP_BUNDLE/dmg/"*.dmg ]; then
            cp "$DESKTOP_BUNDLE/dmg/"*.dmg "$RELEASE_DIR/BridgeX-$VERSION.dmg"
            echo -e "${GREEN}✓ Copied DMG${NC}"
        fi
        
        # Copy APP
        if [ -d "$DESKTOP_BUNDLE/macos/"*.app ]; then
            cp -r "$DESKTOP_BUNDLE/macos/"*.app "$RELEASE_DIR/"
            echo -e "${GREEN}✓ Copied APP bundle${NC}"
        fi
        
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        PLATFORM="windows"
        DESKTOP_BUNDLE="desktop/src-tauri/target/release/bundle"
        
        # Copy NSIS installer
        if [ -f "$DESKTOP_BUNDLE/nsis/"*-setup.exe ]; then
            cp "$DESKTOP_BUNDLE/nsis/"*-setup.exe "$RELEASE_DIR/BridgeX-$VERSION-setup.exe"
            echo -e "${GREEN}✓ Copied NSIS installer${NC}"
        fi
        
        # Copy MSI
        if [ -f "$DESKTOP_BUNDLE/msi/"*.msi ]; then
            cp "$DESKTOP_BUNDLE/msi/"*.msi "$RELEASE_DIR/BridgeX-$VERSION.msi"
            echo -e "${GREEN}✓ Copied MSI installer${NC}"
        fi
    fi
    
    # Copy Android APK
    if [ -f "mobile/build/app/outputs/flutter-apk/app-release.apk" ]; then
        cp mobile/build/app/outputs/flutter-apk/app-release.apk "$RELEASE_DIR/BridgeX-$VERSION.apk"
        echo -e "${GREEN}✓ Copied Android APK${NC}"
    fi
    
    # Copy backend binary
    if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
        cp backend/target/release/bridgex-server.exe "$RELEASE_DIR/"
    else
        cp backend/target/release/bridgex-server "$RELEASE_DIR/"
    fi
    echo -e "${GREEN}✓ Copied backend binary${NC}"
    
    echo ""
}

# Function to generate checksums
generate_checksums() {
    echo -e "${YELLOW}Generating checksums...${NC}"
    cd "$RELEASE_DIR"
    
    if command -v sha256sum &> /dev/null; then
        sha256sum * > SHA256SUMS.txt
    elif command -v shasum &> /dev/null; then
        shasum -a 256 * > SHA256SUMS.txt
    fi
    
    cd ../..
    echo -e "${GREEN}✓ Checksums generated${NC}"
    echo ""
}

# Function to create release notes
create_release_notes() {
    echo -e "${YELLOW}Creating release notes...${NC}"
    
    cat > "$RELEASE_DIR/RELEASE_NOTES.md" << EOF
# BridgeX $VERSION

## 📥 Installation

See [INSTALL.md](https://github.com/Youni-G5/bridge-x/blob/main/INSTALL.md) for detailed instructions.

### Quick Links

- **Windows**: Download \`BridgeX-$VERSION-setup.exe\`
- **macOS**: Download \`BridgeX-$VERSION.dmg\`
- **Linux**: Download \`BridgeX-$VERSION.AppImage\` or \`.deb\`
- **Android**: Download \`BridgeX-$VERSION.apk\`

## ✨ What's New

- Auto-update support
- Improved installer UX
- Desktop file picker functional
- Drag-and-drop file support
- Better error messages
- System tray integration
- Background service mode

## 🔐 Security

All binaries are built from source via GitHub Actions.
Verify checksums with \`SHA256SUMS.txt\`.

## 📝 Full Changelog

See [CHANGELOG.md](https://github.com/Youni-G5/bridge-x/blob/main/CHANGELOG.md)
EOF

    echo -e "${GREEN}✓ Release notes created${NC}"
    echo ""
}

# Main execution
echo -e "${YELLOW}Starting build process...${NC}"
echo ""

build_backend
build_desktop
build_mobile
collect_artifacts
generate_checksums
create_release_notes

echo "====================================="
echo -e "${GREEN}✓ Build complete!${NC}"
echo "====================================="
echo ""
echo -e "Release artifacts in: ${GREEN}$RELEASE_DIR${NC}"
echo ""
echo "Files:"
ls -lh "$RELEASE_DIR"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Test the installers on target platforms"
echo "2. Create a git tag: git tag v$VERSION"
echo "3. Push tag: git push origin v$VERSION"
echo "4. GitHub Actions will create the release automatically"
echo ""
echo -e "${GREEN}Happy releasing! 🚀${NC}"
