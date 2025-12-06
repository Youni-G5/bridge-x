# 🚀 Quick Start - BridgeX

Guide rapide pour lancer BridgeX en **moins de 5 minutes**.

---

## 💻 Pour Développeurs

### Prérequis

```bash
# Rust (backend + desktop)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Flutter (mobile)
# Télécharger depuis https://flutter.dev

# Node.js (Tauri)
# Télécharger depuis https://nodejs.org
```

### Installation Développement

```bash
# 1. Cloner le repo
git clone https://github.com/Youni-G5/bridge-x.git
cd bridge-x

# 2. Installer toutes les dépendances
chmod +x scripts/*.sh
./scripts/bootstrap.sh

# 3. Lancer en mode dev
./scripts/run_local.sh
```

C'est tout ! 🎉

### Architecture Rapide

```
bridge-x/
├── backend/          # Serveur Rust (Axum + SQLite)
├── desktop/          # App Tauri (Rust + Web)
├── mobile/           # App Flutter (Dart)
├── docs/             # Documentation
└── scripts/          # Scripts utilitaires
```

### Commandes Utiles

```bash
# Backend seul
cd backend
cargo run --release

# Desktop seul
cd desktop
cargo tauri dev

# Mobile seul
cd mobile
flutter run

# Tests
cd backend && cargo test
cd mobile && flutter test

# Build production
./scripts/build_release.sh
```

---

## 👥 Pour Utilisateurs

### Installation

**Option 1 : Télécharger depuis GitHub**

1. Aller sur [Releases](https://github.com/Youni-G5/bridge-x/releases/latest)
2. Télécharger l'installer pour votre système :
   - Windows : `BridgeX-setup.exe`
   - macOS : `BridgeX.dmg`
   - Linux : `BridgeX.AppImage` ou `.deb`
   - Android : `BridgeX.apk`
3. Installer et lancer

**Option 2 : Compiler depuis source**

Voir section "Pour Développeurs" ci-dessus.

### Utilisation

#### 👉 Étape 1 : Installer sur 2 appareils

- Sur votre **PC** : Installer BridgeX desktop
- Sur votre **mobile** : Installer BridgeX APK

#### 👉 Étape 2 : Appairer

1. **Sur PC** : Ouvrir BridgeX → Cliquer "Pair Device" → QR code s'affiche
2. **Sur Mobile** : Ouvrir BridgeX → "Scan QR" → Scanner le QR du PC
3. ✅ **Connecté !**

#### 👉 Étape 3 : Transférer

**De mobile vers PC** :
- Sélectionner le PC → "Send Files" → Choisir fichiers → Envoyer

**De PC vers mobile** :
- Glisser un fichier dans la fenêtre BridgeX **OU**
- Cliquer "Send" → Choisir fichier → Sélectionner mobile destination

**C'est tout ! Tous les transferts sont chiffrés automatiquement.** 🔒

---

## ❓ Troubleshooting

### Desktop ne se lance pas

```bash
# Vérifier que le backend est bien bundle
ls desktop/src-tauri/target/release/bridgex-server*

# Si manquant, compiler le backend d'abord
cd backend
cargo build --release

# Copier dans desktop
cp target/release/bridgex-server ../desktop/src-tauri/
```

### Mobile ne trouve pas le PC

1. **Vérifier** : PC et mobile sur le **même WiFi**
2. **Vérifier** : Firewall autorise port `8080`
3. **Vérifier** : Backend tourne sur PC (voir logs)
4. **Essayer** : Scanner QR à nouveau

### Erreur "Backend not found"

```bash
# Sur Windows
cd backend
cargo build --release
copy target\release\bridgex-server.exe ..\desktop\src-tauri\

# Sur macOS/Linux
cd backend
cargo build --release
cp target/release/bridgex-server ../desktop/src-tauri/
```

### Compilation échoue

```bash
# Mettre à jour Rust
rustup update

# Nettoyer et rebuild
cargo clean
cargo build --release

# Si Flutter
flutter clean
flutter pub get
```

---

## 📚 Ressources

### Documentation
- [Installation Détaillée](INSTALL.md) - Guide complet tous OS
- [Sécurité](SECURITY.md) - Modèle de menaces et crypto
- [Architecture](docs/architecture.md) - Design système
- [API](docs/openapi.yaml) - Spécification REST
- [Contributing](CONTRIBUTING.md) - Comment contribuer

### Support
- 🐛 [Issues](https://github.com/Youni-G5/bridge-x/issues) - Reporter bugs
- 💬 [Discussions](https://github.com/Youni-G5/bridge-x/discussions) - Questions
- 🔒 [Security](SECURITY.md) - Vulnérabilités

### Communauté
- ⭐ [Star le projet](https://github.com/Youni-G5/bridge-x) si tu aimes !
- 👥 [Contribuer](CONTRIBUTING.md) - PRs bienvenues
- 🐦 [Twitter](https://twitter.com/bridgex_app) - Suivre les news

---

## 🎯 Fonctionnalités Clés

✅ **Chiffrement E2E** - X25519 + AES-256-GCM  
✅ **Local seulement** - Aucun serveur cloud  
✅ **Open source** - Code auditable  
✅ **Multi-plateforme** - Windows, macOS, Linux, Android  
✅ **Zéro config** - Fonctionne immédiatement  
✅ **Pairing rapide** - 5 secondes avec QR  
✅ **Drag & drop** - Glisser fichiers facilement  
✅ **Aucun compte** - Pas d'inscription

---

## 🛣️ Roadmap

### v0.1.0 (Actuel) ✅
- Transfert fichiers P2P
- Chiffrement E2E
- Apps desktop + mobile
- Pairing QR code

### v0.2.0 (Janvier 2026)
- Reprise transferts interrompus
- Service background mobile
- Historique transferts

### v0.5.0 (Q1 2026)
- Sync clipboard
- App iOS
- WebRTC P2P (accès remote)
- Multi-langue

### v1.0.0 (Q2 2026)
- Partage d'écran
- Contrôle remote
- App stores (Google Play, Apple)

---

## 📝 Licence

MIT License - Voir [LICENSE](LICENSE)

---

**Fait avec ❤️ par la communauté BridgeX**

[GitHub](https://github.com/Youni-G5/bridge-x) • [Website](https://bridgex.dev) • [Docs](docs/) • [Discord](https://discord.gg/bridgex)
