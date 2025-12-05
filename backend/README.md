# BridgeX Backend

Rust-based backend server for BridgeX P2P file transfer and device pairing.

## Features

- **REST API**: HTTP endpoints for device management
- **P2P Connectivity**: WebRTC with TCP/WebSocket fallback
- **E2E Encryption**: X25519 key exchange + AES-256-GCM
- **SQLite Storage**: Local database for device pairings
- **QR Code Pairing**: Secure device pairing via QR codes

## API Endpoints

### Health Check
```
GET /api/v1/health
```
Returns server status and version.

### Device Pairing
```
POST /api/v1/pair
Content-Type: application/json

{
  "device_name": "My Phone"
}
```
Generates keypair and QR code for pairing.

### Initialize Transfer
```
POST /api/v1/transfer
Content-Type: application/json

{
  "device_id": "uuid",
  "file_name": "document.pdf",
  "file_size": 1024000,
  "file_hash": "sha256..."
}
```
Creates transfer session.

### Server Status
```
GET /api/v1/status
```
Returns active connections and transfer stats.

## Development

### Prerequisites

- Rust 1.70+
- SQLite 3

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

Server starts on `http://127.0.0.1:8080`

### Test

```bash
cargo test
```

### Lint

```bash
cargo clippy
cargo fmt --check
```

## Configuration

Create `.env` file (see `.env.example`):

```env
BRIDGEX_PORT=8080
BRIDGEX_HOST=127.0.0.1
BRIDGEX_DB_PATH=./data/bridge.db
BRIDGEX_LOG_LEVEL=info
```

## Architecture

```
backend/
├── src/
│   ├── main.rs           # Server entry point
│   ├── lib.rs            # Library exports
│   ├── server/
│   │   ├── mod.rs        # Server module
│   │   ├── api.rs        # REST API handlers
│   │   └── p2p.rs        # P2P connection logic
│   ├── crypto/
│   │   └── keys.rs       # Cryptography (X25519, ECDH)
│   └── util.rs           # Utility functions
└── tests/
    └── integration_test.rs  # Integration tests
```

## Security

### Encryption

- **Key Exchange**: X25519 ECDH
- **Session Keys**: Derived via HKDF-SHA256
- **Data Encryption**: AES-256-GCM
- **Perfect Forward Secrecy**: Keys rotate per session

### Best Practices

- Never commit `.env` file
- Use GitHub Secrets for CI/CD
- Run `cargo audit` regularly
- Keep dependencies updated

## Performance

- Async I/O with Tokio
- Connection pooling
- Efficient buffer management
- Zero-copy where possible

## License

MIT
