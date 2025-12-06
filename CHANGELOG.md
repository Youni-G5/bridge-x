# Changelog

All notable changes to BridgeX will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-06

### 🎉 First Public Release - MVP Complete!

BridgeX is now ready for public use! Download installers for Windows, macOS, Linux, and Android.

### Added

#### Backend (Rust + Axum)
- ✅ REST API with health checks, device pairing, file transfers
- ✅ End-to-end encryption (X25519 + AES-256-GCM + HKDF-SHA256)
- ✅ SQLite database for device and transfer management
- ✅ QR code generation (SVG and PNG formats)
- ✅ Chunked file upload support (1MB chunks)
- ✅ Session-based authentication
- ✅ Comprehensive error handling

#### Desktop App (Tauri)
- ✅ Native application for Windows, macOS, and Linux
- ✅ **Backend auto-start** - Zero configuration required!
- ✅ **File picker integration** - Select files/folders easily
- ✅ **Drag & drop support** - Drop files directly into the window
- ✅ Device pairing via QR code display
- ✅ Device list management
- ✅ File transfer with progress tracking
- ✅ System tray integration
- ✅ Auto-update support (Tauri updater)
- ✅ Health check monitoring
- ✅ Clean shutdown handling

#### Mobile App (Flutter)
- ✅ Native Android application
- ✅ **QR code scanner** - Pair devices in 5 seconds
- ✅ **File picker** - Send files from mobile to PC
- ✅ Device discovery on local network
- ✅ File transfer with progress bars
- ✅ Multi-file selection support
- ✅ Beautiful Material Design UI
- ✅ Secure storage for credentials
- ✅ Error handling with user-friendly messages

#### Documentation
- ✅ Comprehensive README with download buttons
- ✅ Detailed installation guide (INSTALL.md) for all platforms
- ✅ Security documentation (SECURITY.md)
- ✅ Architecture documentation
- ✅ OpenAPI specification for REST API
- ✅ Contributing guidelines
- ✅ Code of conduct

#### CI/CD & Automation
- ✅ GitHub Actions workflows for Rust, Flutter, and Tauri
- ✅ Automated testing on push and PR
- ✅ Release workflow with automatic installer generation
- ✅ Build script for all platforms (`build_release.sh`)
- ✅ Dependabot for dependency updates

### Security
- ✅ X25519 Elliptic Curve Diffie-Hellman key exchange
- ✅ AES-256-GCM authenticated encryption
- ✅ HKDF-SHA256 session key derivation
- ✅ Forward secrecy (new keys per session)
- ✅ Local-only transfers (no cloud)
- ✅ No account required
- ✅ Open source and auditable

### Performance
- ⚡ Fast local transfers (WiFi speed, typically 10-50 MB/s)
- ⚡ Efficient chunked uploads
- ⚡ Low memory footprint (~50MB RAM desktop, ~30MB mobile)
- ⚡ SQLite optimized queries
- ⚡ Async/await architecture (non-blocking I/O)

### Platform Support
- ✅ **Windows** - `.exe` installer with NSIS
- ✅ **macOS** - `.dmg` installer
- ✅ **Linux** - `.AppImage` and `.deb` packages
- ✅ **Android** - `.apk` direct download
- 🚧 **iOS** - Coming soon (TestFlight beta)

### Known Limitations
- ⚠️ No transfer resume on connection loss (planned for v0.2.0)
- ⚠️ No background transfers on mobile (app must stay active)
- ⚠️ No clipboard sync yet (planned for v0.5.0)
- ⚠️ No remote access over internet (local network only)
- ⚠️ iOS app not available yet (requires Apple Developer account)

### Installation

**Desktop (Windows/macOS/Linux):**
1. Download installer from [GitHub Releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. Run installer
3. Launch BridgeX

**Mobile (Android):**
1. Download `.apk` from [GitHub Releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. Enable "Install from unknown sources" if needed
3. Install and open

See [INSTALL.md](INSTALL.md) for detailed instructions.

### Upgrade Notes

This is the first public release, no upgrade path needed.

---

## [Unreleased]

### Planned for v0.2.0 (January 2026)
- [ ] Transfer resume support (continue interrupted transfers)
- [ ] Background service for mobile (Android)
- [ ] File transfer history UI
- [ ] Multiple file selection improvements
- [ ] Notification system (push notifications mobile)
- [ ] Transfer speed optimizations

### Planned for v0.5.0 (Q1 2026)
- [ ] Clipboard synchronization
- [ ] WebRTC P2P (NAT traversal for remote access)
- [ ] Folder synchronization
- [ ] iOS app (TestFlight beta)
- [ ] Multi-language support (FR, EN, ES, DE, AR)
- [ ] Dark mode

### Planned for v1.0.0 (Q2 2026)
- [ ] Screen sharing
- [ ] Remote desktop control
- [ ] Plugin system
- [ ] Windows/macOS/Linux code signing
- [ ] Google Play Store release
- [ ] Apple App Store release

---

## Version History

- **0.1.0** (2025-12-06): First public release - MVP complete 🎉
- **0.2.0** (Planned): Transfer resume + background service
- **0.5.0** (Planned): Clipboard sync + iOS + WebRTC
- **1.0.0** (Planned): Screen sharing + app store releases

## Release Process

1. Update version in `Cargo.toml`, `pubspec.yaml`, `tauri.conf.json`, `package.json`
2. Update this CHANGELOG.md with all changes
3. Commit changes: `git commit -am "chore: release vX.Y.Z"`
4. Create git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"`
5. Push with tags: `git push origin main --tags`
6. GitHub Actions will automatically build and create release with installers
7. Edit release notes on GitHub with screenshots and highlights

## Links

- [GitHub Repository](https://github.com/Youni-G5/bridge-x)
- [Releases](https://github.com/Youni-G5/bridge-x/releases)
- [Issue Tracker](https://github.com/Youni-G5/bridge-x/issues)
- [Discussions](https://github.com/Youni-G5/bridge-x/discussions)
- [Documentation](https://github.com/Youni-G5/bridge-x/tree/main/docs)
- [Security Policy](https://github.com/Youni-G5/bridge-x/blob/main/SECURITY.md)

---

**Note**: This project follows [Semantic Versioning](https://semver.org/).

**Contributors**: See [GitHub Contributors](https://github.com/Youni-G5/bridge-x/graphs/contributors)

**License**: MIT - See [LICENSE](LICENSE)
