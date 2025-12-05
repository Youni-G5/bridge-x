# BridgeX 🌉

[![CI Rust](https://github.com/Youni-G5/bridge-x/workflows/CI%20Rust/badge.svg)](https://github.com/Youni-G5/bridge-x/actions)
[![CI Flutter](https://github.com/Youni-G5/bridge-x/workflows/CI%20Flutter/badge.svg)](https://github.com/Youni-G5/bridge-x/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Local-first P2P bridge to sync files, clipboards and tools between PC and mobile**

## 🎯 Overview

BridgeX is a secure, privacy-focused tool for seamless file transfer, clipboard synchronization, and remote control between your PC and mobile devices. Built with Rust, Tauri, and Flutter, it prioritizes local-first architecture with end-to-end encryption.

### Key Features

- 📁 **File Transfer**: Fast P2P file sharing over LAN
- 📋 **Clipboard Sync**: Multi-format clipboard synchronization
- 🖥️ **Remote Explorer**: Browse and manage files remotely
- 💻 **Remote Terminal**: Execute commands on your PC from mobile
- 📷 **OCR & Scan**: Convert photos to text or PDF
- 🎯 **DropZone**: Drag & drop files across devices
- 📦 **TempBox**: Temporary file sharing with expiration
- 📊 **System Monitor**: Real-time system stats

### Architecture

```
┌─────────────────┐         P2P Connection          ┌─────────────────┐
│   Desktop App   │◄──────(WebRTC/QUIC/TCP)────────►│   Mobile App    │
│  (Tauri/Rust)   │         E2E Encrypted            │   (Flutter)     │
└────────┬────────┘                                  └────────┬────────┘
         │                                                    │
         └────────────────┬──────────────┬──────────────────┘
                          │              │
                  ┌───────▼──────┐  ┌────▼─────┐
                  │   Backend    │  │ Optional │
                  │   (Rust)     │  │  Relay   │
                  │   Local API  │  │  Server  │
                  └──────────────┘  └──────────┘
```

## 🚀 Quick Start

### Prerequisites

- **Rust** (1.70+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Flutter** (3.10+): [Install Flutter](https://flutter.dev/docs/get-started/install)
- **Node.js** (16+): For Tauri frontend
- **Tauri CLI**: `cargo install tauri-cli`

### Bootstrap

```bash
chmod +x scripts/bootstrap.sh
./scripts/bootstrap.sh
```

### Running Locally

#### Backend
```bash
cd backend
cargo run
# Server starts on http://127.0.0.1:8080
```

#### Desktop
```bash
cd desktop
cargo tauri dev
```

#### Mobile
```bash
cd mobile
flutter pub get
flutter run
```

## 🛣️ Roadmap

### MVP (v0.1)
- [x] Repository setup
- [ ] Backend health endpoint
- [ ] Pairing via QR code
- [ ] Basic file transfer
- [ ] Mobile QR scanner
- [ ] Desktop device list

### Beta (v0.5)
- [ ] WebRTC P2P
- [ ] Clipboard sync
- [ ] E2E encryption
- [ ] Remote file explorer

### v1.0
- [ ] Remote terminal
- [ ] OCR & scanning
- [ ] System monitoring

## 🔒 Security

- **E2E Encryption**: X25519 + AES-256-GCM
- **Local-first**: No cloud required
- **Zero-knowledge**: Your data stays yours

See [SECURITY.md](SECURITY.md) for details.

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

MIT License - see [LICENSE](LICENSE)

---

**Built with ❤️ for privacy-conscious users**
