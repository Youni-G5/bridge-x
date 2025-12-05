# Security Policy

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in BridgeX, please report it responsibly.

### How to Report

1. **DO NOT** open a public issue for security vulnerabilities
2. Send details to: [open an issue with "SECURITY" prefix]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Status Updates**: Regular updates on progress
- **Resolution**: Security patch released as priority

### Disclosure Policy

- We will coordinate disclosure timing with you
- Credit will be given to reporters (if desired)
- CVE may be requested for significant issues

## Security Best Practices

### For Users

#### Key Management

- **Never share private keys** between devices manually
- Device keys are stored encrypted locally
- Use pairing QR codes only in trusted environments
- Verify device fingerprints when pairing

#### Network Security

- Use trusted networks for initial pairing
- All data is E2E encrypted regardless of network
- Consider using VPN for additional privacy

#### Data Protection

- BridgeX stores minimal data locally (only device pairings)
- No data sent to cloud by default
- Clear pairing history to unpair devices

### For Developers

#### Secrets Management

```bash
# NEVER commit secrets
echo "my-secret-key" > .env  # ❌ BAD

# Use .env.example for templates
BRIDGEX_SECRET_KEY=your-key-here  # ✅ GOOD (.env.example)
```

#### GitHub Secrets

For CI/CD, use GitHub Secrets:

```yaml
# In GitHub Actions
env:
  SIGNING_KEY: ${{ secrets.BRIDGEX_SIGNING_KEY }}
```

Add secrets at: `Settings → Secrets and variables → Actions`

#### Dependency Security

```bash
# Check Rust dependencies
cargo audit

# Update dependencies
cargo update

# Check Flutter dependencies
flutter pub outdated
```

#### Code Review Checklist

- [ ] No hardcoded credentials
- [ ] Input validation on all endpoints
- [ ] SQL injection prevention (use parameterized queries)
- [ ] XSS prevention (sanitize user inputs)
- [ ] CSRF tokens for state-changing operations
- [ ] Rate limiting on API endpoints
- [ ] Proper error handling (no sensitive info leaks)

## Encryption Details

### Key Exchange

BridgeX uses **X25519** elliptic curve for Diffie-Hellman key exchange:

```
1. Each device generates x25519 key pair (private + public)
2. Devices exchange public keys via QR code
3. Both compute shared secret using ECDH
4. Shared secret derives session key (HKDF)
```

### Data Encryption

All transferred data is encrypted with **AES-256-GCM**:

```
Plaintext → AES-256-GCM(session_key, nonce) → Ciphertext + Auth Tag
```

- **Key Size**: 256 bits
- **Mode**: GCM (Galois/Counter Mode) for authenticated encryption
- **Nonce**: Random 96-bit nonce per message
- **Auth Tag**: 128-bit authentication tag

### Perfect Forward Secrecy

Session keys rotate:
- New key derived for each session
- Old keys destroyed after use
- Compromise of one key doesn't affect past/future sessions

## Threat Model

### Protected Against

✅ **Network Eavesdropping**: E2E encryption prevents passive monitoring  
✅ **Man-in-the-Middle**: Key verification during pairing  
✅ **Replay Attacks**: Nonces prevent message replay  
✅ **Unauthorized Access**: Pairing required before communication  
✅ **Data Leaks**: Local-first, no cloud storage  

### NOT Protected Against

⚠️ **Compromised Device**: Malware on device can access decrypted data  
⚠️ **Physical Access**: Attacker with device access can extract keys  
⚠️ **Shoulder Surfing**: QR code visible during pairing  
⚠️ **Social Engineering**: User tricked into pairing malicious device  

## Key Rotation

### When to Rotate Keys

- Suspected compromise
- Device lost/stolen
- Periodic rotation (recommended: every 90 days)
- After sharing device temporarily

### How to Rotate

```bash
# Re-pair devices (generates new keys)
1. Unpair device in settings
2. Scan new QR code to re-pair
3. Old session keys automatically invalidated
```

## Audit Log

BridgeX logs security-relevant events:

- Device pairings/unpairings
- Failed authentication attempts
- Key generation events
- File transfers (metadata only)

Logs stored locally, never transmitted.

## Compliance

### GDPR Compliance

- **Data Minimization**: Only essential data stored
- **User Control**: Users own and control all data
- **Right to Erasure**: Unpair devices to delete data
- **No Tracking**: No analytics or telemetry by default

### Security Certifications

BridgeX is currently **not certified**. Future plans:
- SOC 2 Type II (for enterprise users)
- ISO 27001 (information security)

## Security Roadmap

### Completed
- [x] E2E encryption (AES-GCM)
- [x] Secure key exchange (X25519)
- [x] Local-first architecture

### Planned
- [ ] Hardware security module (HSM) support
- [ ] Biometric authentication
- [ ] Tamper detection
- [ ] Secure enclave integration (iOS/Android)
- [ ] Zero-knowledge proof for pairing
- [ ] Post-quantum cryptography (ML-KEM)

## Dependencies Security

### Critical Dependencies

**Rust:**
- `ring` or `x25519-dalek` — cryptography
- `rustls` — TLS implementation
- `tokio` — async runtime

**Flutter:**
- `flutter_secure_storage` — keychain/keystore
- `qr_code_scanner` — QR scanning

### Vulnerability Monitoring

We use:
- **Dependabot**: Automated dependency updates
- **cargo-audit**: Rust vulnerability scanning
- **Snyk**: Continuous security monitoring

## Contact

For security concerns: Open an issue with `[SECURITY]` prefix

---

**Last Updated**: 2025-12-05  
**Security Policy Version**: 1.0
