# BridgeX Desktop

Tauri-based desktop application for BridgeX.

## Features

- Native desktop UI (Windows, macOS, Linux)
- Device pairing via QR code display
- File sending to mobile devices
- System tray integration (coming soon)
- Auto-start on boot (coming soon)

## Prerequisites

- Rust 1.70+
- Node.js 16+ (optional, for advanced frontend)
- Tauri CLI: `cargo install tauri-cli`

## Development

### Install Dependencies

```bash
cd desktop
cargo build
```

### Run Dev Mode

```bash
cargo tauri dev
```

This opens the app with hot-reload enabled.

### Build Release

```bash
cargo tauri build
```

Outputs:
- **Windows**: `target/release/bundle/msi/BridgeX_0.1.0_x64_en-US.msi`
- **macOS**: `target/release/bundle/dmg/BridgeX_0.1.0_x64.dmg`
- **Linux**: `target/release/bundle/deb/bridgex_0.1.0_amd64.deb`

## Project Structure

```
desktop/
├── src-tauri/
│   ├── Cargo.toml          # Rust dependencies
│   ├── tauri.conf.json     # Tauri configuration
│   └── src/
│       └── main.rs         # Rust backend (Tauri commands)
└── src/
    ├── index.html          # Frontend UI
    ├── styles.css          # Styles
    └── main.js             # Frontend logic
```

## Tauri Commands

### check_health

Checks backend server health.

```javascript
const result = await invoke('check_health');
```

### pair_device

Initiates device pairing.

```javascript
const result = await invoke('pair_device', { deviceName: 'My Phone' });
```

### get_devices

Retrieves list of paired devices.

```javascript
const devices = await invoke('get_devices');
```

### send_file

Initiates file transfer.

```javascript
const result = await invoke('send_file', { 
    deviceId: 'uuid', 
    filePath: '/path/to/file' 
});
```

## Configuration

Edit `src-tauri/tauri.conf.json`:

```json
{
  "productName": "BridgeX",
  "version": "0.1.0",
  "identifier": "com.bridgex.app"
}
```

## Building for Distribution

### Windows

```bash
cargo tauri build --target x86_64-pc-windows-msvc
```

Creates `.msi` installer.

### macOS

```bash
cargo tauri build --target x86_64-apple-darwin
# or for Apple Silicon
cargo tauri build --target aarch64-apple-darwin
```

Creates `.dmg` and `.app` bundle.

### Linux

```bash
cargo tauri build --target x86_64-unknown-linux-gnu
```

Creates `.deb`, `.AppImage`, and `.rpm` packages.

## Code Signing (for production)

### macOS

1. Get Apple Developer certificate
2. Set in `tauri.conf.json`:

```json
"macOS": {
  "identity": "Developer ID Application: Your Name (TEAM_ID)"
}
```

### Windows

1. Get code signing certificate
2. Set certificate thumbprint:

```json
"windows": {
  "certificateThumbprint": "YOUR_THUMBPRINT"
}
```

## Troubleshooting

### Build fails on Linux

Install dependencies:

```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
    libappindicator3-dev librsvg2-dev patchelf
```

### Backend not starting

Check that `bridgex-server` binary is in PATH or adjust spawn command in `main.rs`.

### Hot reload not working

Restart dev server:

```bash
cargo tauri dev
```

## License

MIT
