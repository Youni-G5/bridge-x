# BridgeX Architecture

## Overview

BridgeX is a local-first, peer-to-peer file transfer and device control system built with Rust (backend), Tauri (desktop), and Flutter (mobile). All communication is end-to-end encrypted using X25519 + AES-256-GCM.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User Devices                             │
├──────────────────────┬──────────────────────────────────────────┤
│                      │                                          │
│   Desktop (Tauri)    │         Mobile (Flutter)                 │
│   ┌──────────────┐   │         ┌──────────────┐                │
│   │   Frontend   │   │         │   Flutter    │                │
│   │  (HTML/CSS)  │   │         │     App      │                │
│   └──────┬───────┘   │         └──────┬───────┘                │
│          │           │                │                        │
│   ┌──────▼───────┐   │         ┌──────▼───────┐                │
│   │Tauri Backend │   │         │  API Client  │                │
│   │  (Rust IPC)  │   │         │    (HTTP)    │                │
│   └──────┬───────┘   │         └──────┬───────┘                │
│          │           │                │                        │
└──────────┼───────────┴────────────────┼────────────────────────┘
           │                            │
           │    ┌───────────────────────┘
           │    │
           ▼    ▼
    ┌──────────────────┐
    │  Backend Server  │
    │  (Rust + Axum)   │
    │                  │
    │  ┌────────────┐  │
    │  │ REST API   │  │
    │  ├────────────┤  │
    │  │ P2P Engine │  │
    │  ├────────────┤  │
    │  │ Crypto     │  │
    │  ├────────────┤  │
    │  │ Storage    │  │
    │  │ (SQLite)   │  │
    │  └────────────┘  │
    └──────────────────┘
           │
           ▼
    ┌──────────────────┐
    │  P2P Connection  │
    │  (WebRTC/QUIC)   │
    └──────────────────┘
```

## Components

### 1. Backend Server (Rust)

**Responsibilities:**
- HTTP REST API for device management
- P2P connection management (WebRTC signaling)
- Cryptographic operations (key generation, encryption)
- Data persistence (device pairings, settings)
- File transfer coordination

**Key Modules:**
- `server::api` - REST endpoints
- `server::p2p` - P2P connection logic
- `crypto::keys` - Key management and encryption
- `util` - Helper functions

**Technologies:**
- **Axum** - Async HTTP framework
- **Tokio** - Async runtime
- **SQLx** - Database access
- **x25519-dalek** - Elliptic curve cryptography
- **aes-gcm** - Symmetric encryption

### 2. Desktop App (Tauri)

**Responsibilities:**
- Native desktop UI
- Spawn and manage backend server
- Display QR codes for pairing
- File selection and transfer initiation
- System tray integration

**Architecture:**
```
┌─────────────────────────────────┐
│         Frontend (Web)          │
│    HTML + CSS + JavaScript      │
└────────────┬────────────────────┘
             │ Tauri IPC
             ▼
┌─────────────────────────────────┐
│       Tauri Backend (Rust)      │
│  - Invoke handlers              │
│  - Process spawning             │
│  - File system access           │
└─────────────────────────────────┘
```

### 3. Mobile App (Flutter)

**Responsibilities:**
- QR code scanning for pairing
- File selection (gallery, documents)
- Transfer progress tracking
- Device management UI
- Push notifications (future)

**Architecture:**
```
┌─────────────────────────────────┐
│         UI Layer (Screens)      │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│   Services (BridgeService)      │
│  - API client (HTTP)            │
│  - WebSocket connection         │
│  - State management             │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│        Models (Device)          │
└─────────────────────────────────┘
```

## Data Flow

### Pairing Flow

```
Desktop                Backend              Mobile
   │                      │                    │
   │──1. Request Pair────▶│                    │
   │   POST /api/v1/pair  │                    │
   │                      │                    │
   │◀──2. QR Data─────────│                    │
   │   {public_key, id}   │                    │
   │                      │                    │
   │                      │◀──3. Scan QR───────│
   │                      │   (mobile app)     │
   │                      │                    │
   │                      │──4. Verify Keys───▶│
   │                      │   ECDH exchange    │
   │                      │                    │
   │◀──5. Paired──────────│──────────────────▶│
   │   Session established│                    │
```

### File Transfer Flow

```
Sender                 Backend              Receiver
   │                      │                    │
   │──1. Init Transfer───▶│                    │
   │   POST /api/v1/transfer                  │
   │   {file_name, size}  │                    │
   │                      │                    │
   │◀──2. Transfer ID─────│                    │
   │   {transfer_id, url} │                    │
   │                      │                    │
   │──3. Upload Chunks───▶│                    │
   │   (encrypted)        │                    │
   │                      │                    │
   │                      │──4. Notify────────▶│
   │                      │   WebSocket push   │
   │                      │                    │
   │                      │◀──5. Download──────│
   │                      │   (encrypted)      │
   │                      │                    │
   │◀──6. Complete────────│──────────────────▶│
```

## Security Architecture

### Key Exchange (X25519 ECDH)

```
Device A                                    Device B
   │                                            │
   │  1. Generate keypair                       │
   │     private_a = random()                   │
   │     public_a = X25519(private_a)           │
   │                                            │
   │──────2. Exchange public keys──────────────▶│
   │         QR: public_a                       │
   │                                            │
   │                      3. Generate keypair   │
   │                         private_b = random()│
   │                         public_b = X25519(private_b)
   │                                            │
   │◀─────4. Send public_b──────────────────────│
   │                                            │
   │  5. Compute shared secret                  │
   │     shared = ECDH(private_a, public_b)     │
   │                                            │
   │                   6. Compute shared secret │
   │                      shared = ECDH(private_b, public_a)
   │                                            │
   │  7. Derive session key                     │
   │     session_key = HKDF-SHA256(shared)      │
   │                                            │
   │                        8. Derive same key  │
   │                           session_key = HKDF-SHA256(shared)
   │                                            │
   │◀════════9. Encrypted communication═══════▶│
   │         AES-256-GCM(session_key)           │
```

### Encryption Layers

1. **Transport Layer**: TLS (HTTPS/WSS) for API communication
2. **Application Layer**: AES-256-GCM for file data
3. **Key Exchange**: X25519 ECDH for session keys

### Key Storage

- **Desktop**: OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service)
- **Mobile**: Flutter Secure Storage (Android Keystore, iOS Keychain)
- **Backend**: Encrypted SQLite database

## Database Schema

### devices table

```sql
CREATE TABLE devices (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL, -- 'desktop', 'mobile', 'tablet'
    public_key BLOB NOT NULL,
    paired_at TIMESTAMP NOT NULL,
    last_seen TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

### transfers table

```sql
CREATE TABLE transfers (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    file_hash TEXT NOT NULL,
    status TEXT NOT NULL, -- 'pending', 'uploading', 'completed', 'failed'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    FOREIGN KEY (device_id) REFERENCES devices(id)
);
```

### sessions table

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    device_id TEXT NOT NULL,
    session_key BLOB NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (device_id) REFERENCES devices(id)
);
```

## API Endpoints

See [openapi.yaml](openapi.yaml) for full specification.

### Core Endpoints

- `GET /api/v1/health` - Health check
- `POST /api/v1/pair` - Device pairing
- `POST /api/v1/transfer` - Initialize transfer
- `GET /api/v1/status` - Server status
- `GET /api/v1/devices` - List paired devices
- `DELETE /api/v1/devices/{id}` - Unpair device

## P2P Connection Strategy

### Priority Order

1. **WebRTC (Direct)** - Lowest latency, NAT traversal
2. **QUIC** - UDP-based, fast connection
3. **TCP** - Reliable fallback
4. **WebSocket** - Final fallback, works everywhere

### NAT Traversal

- **STUN** servers for public IP discovery
- **TURN** relay servers for restrictive NATs (optional)
- **ICE** candidate gathering
- **mDNS** for local network discovery

## Deployment Options

### 1. Local-First (Default)

- Backend runs on desktop machine
- Mobile connects via LAN
- No internet required after initial setup

### 2. Self-Hosted Relay

- Deploy backend on VPS
- Mobile connects via internet
- Useful for remote access

See [HOWTO_SELFHOST.md](HOWTO_SELFHOST.md) for instructions.

### 3. Hybrid Mode

- Local connection preferred
- Relay fallback when on different networks

## Performance Considerations

### File Transfer Optimization

- **Chunked uploads**: 1MB chunks for progress tracking
- **Parallel transfers**: Up to 4 concurrent files
- **Compression**: Optional gzip for text files
- **Resume support**: Transfers can be resumed after interruption

### Memory Management

- Streaming file transfers (no full file in memory)
- Connection pooling
- Efficient buffer management

### Network Optimization

- Auto-detect best P2P method
- Adaptive bitrate for slow connections
- Local network prioritization

## Error Handling

### Error Categories

1. **Network Errors**: Connection failures, timeouts
2. **Crypto Errors**: Key verification failures
3. **Storage Errors**: Disk full, permission denied
4. **Protocol Errors**: Invalid data format

### Retry Strategy

- Exponential backoff: 1s, 2s, 4s, 8s, 16s
- Max retries: 5
- Circuit breaker for persistent failures

## Monitoring & Logging

### Log Levels

- **ERROR**: Critical failures requiring attention
- **WARN**: Degraded functionality
- **INFO**: Key operations (pairing, transfers)
- **DEBUG**: Detailed troubleshooting info
- **TRACE**: Very verbose (development only)

### Metrics (Future)

- Transfer success rate
- Average transfer speed
- Connection type distribution
- Error rates by category

## Future Architecture Enhancements

### Phase 2

- Multi-device support (1-to-many transfers)
- Group transfers
- Encrypted cloud backup sync

### Phase 3

- End-to-end encrypted chat
- Screen mirroring
- Remote desktop control

### Phase 4

- Plugin system for extensions
- Third-party integrations (Dropbox, Google Drive)
- Blockchain-based identity verification (optional)

## References

- [X25519 Specification](https://cr.yp.to/ecdh.html)
- [AES-GCM](https://en.wikipedia.org/wiki/Galois/Counter_Mode)
- [WebRTC](https://webrtc.org/)
- [Tauri](https://tauri.app/)
- [Flutter](https://flutter.dev/)
