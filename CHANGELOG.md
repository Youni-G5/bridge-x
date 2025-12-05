# Changelog

All notable changes to BridgeX will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Backend REST API (Rust + Axum)
  - Health check endpoint
  - Device pairing with QR code generation
  - File transfer initialization
  - Device management (list, delete)
- SQLite database integration
  - Device storage
  - Transfer tracking
  - Session management
- Cryptography module
  - X25519 key generation
  - ECDH key exchange
  - HKDF session key derivation
- QR code generation (SVG and PNG)
- Desktop app (Tauri)
  - Native UI with HTML/CSS/JS
  - Backend process management
  - Device pairing UI
  - Device list display
- Mobile app (Flutter)
  - Home screen
  - Devices screen
  - API client service
  - Device model
- Documentation
  - Architecture documentation
  - OpenAPI specification
  - Self-hosting guide
  - Component READMEs
- CI/CD
  - Rust workflow (build, test, lint)
  - Flutter workflow (build APK, test, analyze)
  - Tauri workflow (multi-platform build)
  - Dependabot configuration
- Scripts
  - Bootstrap script (dependency installation)
  - Build all script
  - Local run script
  - Test runner script
- Tests
  - Crypto module unit tests
  - Database integration tests
  - API endpoint tests (skeleton)
  - Flutter widget tests (skeleton)

### Security
- End-to-end encryption ready (X25519 + AES-GCM)
- Secure key storage architecture
- No secrets in repository
- Security policy documented

## [0.1.0] - TBD (MVP Target)

### Planned
- Complete QR scanner implementation (mobile)
- File picker integration (mobile)
- WebRTC P2P implementation (basic)
- Clipboard synchronization (basic)
- Complete test coverage (>80%)
- Release packaging (Windows, macOS, Linux, Android)

---

## Version History

- **Unreleased**: Current development version
- **0.1.0**: MVP release (planned)
- **0.5.0**: Beta release (planned)
- **1.0.0**: Stable release (planned)

## Release Process

1. Update version in `Cargo.toml`, `pubspec.yaml`, `tauri.conf.json`
2. Update this CHANGELOG.md
3. Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions will automatically build and create release

## Links

- [GitHub Repository](https://github.com/Youni-G5/bridge-x)
- [Issue Tracker](https://github.com/Youni-G5/bridge-x/issues)
- [Documentation](https://github.com/Youni-G5/bridge-x/tree/main/docs)
